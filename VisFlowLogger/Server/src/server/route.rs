/*
  1. Save logs
  2. Get logs by operationID
  3. Get operationIDs
  3. Get mermaid diagram by OperationID
  4. Get Graph Data by OperationID
*/
use crate::server::models::app_state::AppState;
use crate::server::models::payload::save_logs_payload::SaveLogsPayload;
use crate::services::graph_generator::api::models::vis_flow_log_entry::{
    VisFlowLogEntry, VisFlowLogEntryLogType,
};
use actix_web::{get, post, web, HttpResponse};

///Save logs
#[post("/")]
pub async fn save_logs(
    body: web::Json<SaveLogsPayload>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    /*
    {
      "operation": [
        {
          "operation_id": "example-operation-1",
          "logs": [
            {
              "operation_id": "example-operation-1",
              "block_name": "main",
              "log_type": "START",
              "log_value": null,
              "sequence": 1
            },
            {
              "operation_id": "example-operation-1",
              "block_name": "main",
              "log_type": "LOG",
              "log_value": "Adding 2 and 1",
              "sequence": 2
            },
            {
              "operation_id": "example-operation-1",
              "block_name": "sum",
              "log_type": "START",
              "log_value": null,
              "sequence": 3
            },
            {
              "operation_id": "example-operation-1",
              "block_name": "sum",
              "log_type": "LOG",
              "log_value": "2 + 1 = 3",
              "sequence": 4
            },
            {
              "operation_id": "example-operation-1",
              "block_name": "sum",
              "log_type": "START",
              "log_value": null,
              "sequence": 5
            },
            {
              "operation_id": "example-operation-1",
              "block_name": "sum",
              "log_type": "LOG",
              "log_value": "1 + 1 = 2",
              "sequence": 6
            },
            {
              "operation_id": "example-operation-1",
              "block_name": "sum",
              "log_type": "END",
              "log_value": null,
              "sequence": 7
            },
            {
              "operation_id": "example-operation-1",
              "block_name": "sum",
              "log_type": "END",
              "log_value": null,
              "sequence": 8
            },
            {
              "operation_id": "example-operation-1",
              "block_name": "main",
              "log_type": "STORE",
              "log_value": "sum = 3",
              "sequence": 9
            },
            {
              "operation_id": "example-operation-1",
              "block_name": "foo",
              "log_type": "START",
              "log_value": null,
              "sequence": 10
            },
            {
              "operation_id": "example-operation-1",
              "block_name": "foo",
              "log_type": "LOG",
              "log_value": "foo called",
              "sequence": 11
            },
            {
              "operation_id": "example-operation-1",
              "block_name": "foo",
              "log_type": "END",
              "log_value": null,
              "sequence": 12
            },
            {
              "operation_id": "example-operation-1",
              "block_name": "main",
              "log_type": "END",
              "log_value": null,
              "sequence": 13
            }
          ]
        }
      ]
    }
    */
    let payload = body.into_inner();
    let all_operations: Vec<String> = payload
        .operation
        .iter()
        .map(|operation| operation.operation_id.clone())
        .collect();
    let all_logs = payload
        .operation
        .iter()
        .flat_map(|x| x.logs.iter())
        .collect();
    let save_logs_future = app_state
        .services
        .persistence
        .vis_flow_log
        .save_log(&all_logs);
    let upsert_ops_future = app_state
        .services
        .persistence
        .vis_flow_op
        .upsert(all_operations);
    let (save_logs_result, upsert_ops_result) = tokio::join!(save_logs_future, upsert_ops_future);
    if !save_logs_result || !upsert_ops_result {
        return HttpResponse::InternalServerError().json("Failed to save logs or operations");
    }
    HttpResponse::Ok().json("Saved logs to database")
}

///Get operation Ids
#[get("/")]
pub async fn get_operations(app_state: web::Data<AppState>) -> HttpResponse {
    //TODO Cursor Pagination with both prev and next reference.
    println!("Getting all operations");
    match app_state
        .services
        .persistence
        .vis_flow_op
        .get_operations()
        .await
    {
        Ok(ops) => HttpResponse::Ok().json(ops),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
#[get("/{operation_id}")]
pub async fn get_logs_by_operation_id(
    operation_id: web::Path<String>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    //TODO simple offset based pagination.
    match app_state
        .services
        .persistence
        .vis_flow_log
        .get_logs_by_operation_id(operation_id.into_inner())
        .await
    {
        Ok(logs) => HttpResponse::Ok().json(logs),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
#[get("/graph/{operation_id}")]
pub async fn get_graphs_by_operation_id(
    operation_id: web::Path<String>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    // Get logs with proper error handling
    let logs = match app_state
        .services
        .persistence
        .vis_flow_log
        .get_logs_by_operation_id(operation_id.into_inner())
        .await
    {
        Ok(logs) => logs,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("Failed to get logs: {}", e))
        }
    };
    //TODO Single iteration or map
    // First validate all log types
    for log in &logs {
        match log.log_type.as_str() {
            "LOG" | "START" | "END" | "STORE" | "EXTERNAL_CALL" | "EXTERNAL_CALL_STORE" => (),
            invalid_type => {
                return HttpResponse::BadRequest()
                    .body(format!("Invalid log_type: {}", invalid_type));
            }
        }
    }

    // Then transform logs to entries (now we know all types are valid)
    let entries: Vec<VisFlowLogEntry> = logs
        .iter()
        .map(|log| VisFlowLogEntry {
            log_type: match log.log_type.as_str() {
                "LOG" => VisFlowLogEntryLogType::Log,
                "START" => VisFlowLogEntryLogType::Start,
                "END" => VisFlowLogEntryLogType::End,
                "STORE" => VisFlowLogEntryLogType::Store,
                "EXTERNAL_CALL" => VisFlowLogEntryLogType::ExternalCall,
                "EXTERNAL_CALL_STORE" => VisFlowLogEntryLogType::ExternalCallStore,
                _ => unreachable!(), // We already validated all types
            },
            log_value: log.log_value.clone(),
            block_name: log.block_name.clone(),
        })
        .collect();

    // Generate graph with error handling
    match app_state
        .services
        .graph_generator
        .graph_generator
        .generate_graph(entries)
    {
        Ok(graph) => HttpResponse::Ok().json(graph),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to generate graph : {}", e))
        }
    }
}

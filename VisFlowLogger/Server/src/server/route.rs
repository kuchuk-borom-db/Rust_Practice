use crate::server::models::app_state::AppState;
use crate::server::models::payload::save_logs_payload::SaveLogsPayload;
use crate::services::diagram_generator::api::models::block::DGBlock;
use crate::services::graph_generator::api::models::vis_flow_log_entry::{
    VisFlowLogEntry, VisFlowLogEntryLogType,
};
use actix_web::{get, post, web, HttpResponse};
use std::collections::HashMap;

#[post("/")]
pub async fn save_logs(
    body: web::Json<SaveLogsPayload>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let payload = body.into_inner();
    let all_operations: Vec<(String, String)> = payload
        .operation
        .iter()
        .map(|operation| {
            (
                operation.operation_id.clone(),
                operation.operation_name.clone(),
            )
        })
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

#[get("/")]
pub async fn get_operations(app_state: web::Data<AppState>) -> HttpResponse {
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

    if let Some(invalid_log) = logs.iter().find(|log| {
        !matches!(
            log.log_type.as_str(),
            "LOG" | "START" | "END" | "STORE" | "EXTERNAL_CALL" | "EXTERNAL_CALL_STORE"
        )
    }) {
        return HttpResponse::BadRequest()
            .body(format!("Invalid log_type: {}", invalid_log.log_type));
    }

    let entries = logs
        .iter()
        .map(|log| VisFlowLogEntry {
            log_type: match log.log_type.as_str() {
                "LOG" => VisFlowLogEntryLogType::Log,
                "START" => VisFlowLogEntryLogType::Start,
                "END" => VisFlowLogEntryLogType::End,
                "STORE" => VisFlowLogEntryLogType::Store,
                "EXTERNAL_CALL" => VisFlowLogEntryLogType::ExternalCall,
                "EXTERNAL_CALL_STORE" => VisFlowLogEntryLogType::ExternalCallStore,
                _ => unreachable!(),
            },
            log_value: log.log_value.clone(),
            block_name: log.block_name.clone(),
        })
        .collect();

    match app_state
        .services
        .graph_generator
        .graph_generator
        .generate_graph(entries)
    {
        Ok(graph) => HttpResponse::Ok().json(graph),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to generate graph: {}", e))
        }
    }
}

#[get("/diagram/mermaid/{operation_id}")]
pub async fn generate_diagram_for_operation(
    operation_id: web::Path<String>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
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

    if let Some(invalid_log) = logs.iter().find(|log| {
        !matches!(
            log.log_type.as_str(),
            "LOG" | "START" | "END" | "STORE" | "EXTERNAL_CALL" | "EXTERNAL_CALL_STORE"
        )
    }) {
        return HttpResponse::BadRequest()
            .body(format!("Invalid log_type: {}", invalid_log.log_type));
    }

    let entries = logs
        .iter()
        .map(|log| VisFlowLogEntry {
            log_type: match log.log_type.as_str() {
                "LOG" => VisFlowLogEntryLogType::Log,
                "START" => VisFlowLogEntryLogType::Start,
                "END" => VisFlowLogEntryLogType::End,
                "STORE" => VisFlowLogEntryLogType::Store,
                "EXTERNAL_CALL" => VisFlowLogEntryLogType::ExternalCall,
                "EXTERNAL_CALL_STORE" => VisFlowLogEntryLogType::ExternalCallStore,
                _ => unreachable!(),
            },
            log_value: log.log_value.clone(),
            block_name: log.block_name.clone(),
        })
        .collect();

    let graph = match app_state
        .services
        .graph_generator
        .graph_generator
        .generate_graph(entries)
    {
        Ok(gg_graph) => gg_graph
            .into_iter()
            .map(|(key, block)| (key, DGBlock::from(block)))
            .collect::<HashMap<String, DGBlock>>(),
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Failed to generate graph: {}", e))
        }
    };

    match app_state
        .services
        .diagram_generator
        .mermaid
        .generate_diagram(graph)
    {
        Ok(diagram) => HttpResponse::Ok().body(diagram),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to generate diagram: {}", e))
        }
    }
}

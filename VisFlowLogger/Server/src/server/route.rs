/*
  1. Save logs
  2. Get logs by operationID
  3. Get operationIDs
  3. Get mermaid diagram by OperationID
  4. Get Graph Data by OperationID
*/

use crate::server::models::app_state::AppState;
use crate::server::models::payload::save_logs_payload::SaveLogsPayload;
use actix_web::{get, post, web, HttpResponse};

///Save logs
#[post("/")]
pub async fn save_logs(
    body: web::Json<SaveLogsPayload>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let payload = body.into_inner();
    let all_operations: Vec<String> = payload
        .operation
        .iter()
        .map(|operation| operation.operation_id.clone())
        .collect();
    let all_logs= payload
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

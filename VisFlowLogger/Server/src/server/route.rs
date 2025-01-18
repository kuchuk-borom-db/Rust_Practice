/*
  1. Save logs
  2. Get logs by operationID
  3. Get operationIDs
  3. Get mermaid diagram by OperationID
  4. Get Graph Data by OperationID
*/

use crate::server::models::app_state::AppState;
use crate::services::persistence::api::model::vis_flow_log_model::VisFlowLogModel;
use actix_web::{get, post, web, HttpResponse};

///Save logs
#[post("/")]
pub async fn save_logs(
    body: web::Json<Vec<VisFlowLogModel>>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    //TODO Move operation_id to server side
    let logs = body.as_ref();
    app_state
        .services
        .persistence
        .vis_flow_log
        .save_log(logs)
        .await;
    app_state
        .services
        .persistence
        .vis_flow_op
        .upsert(&logs[0].operation_id)
        .await;
    HttpResponse::Ok().json("Success")
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

/*
  1. Save logs
  2. Get logs by operationID
  3. Get operationIDs
  3. Get mermaid diagram by OperationID
  4. Get Graph Data by OperationID
*/

use crate::models::app_state::AppState;
use crate::services::persistence::api::model::vis_flow_log_model::VisFlowLogModel;
use actix_web::{post, web, HttpResponse};

#[post("/")]
pub async fn save_logs(
    body: web::Json<Vec<VisFlowLogModel>>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    HttpResponse::Ok().json("Success")
}

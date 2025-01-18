use crate::models::app_state::{AppState, AvailableServices as AS};
use crate::services::persistence::api::services::vis_flow_op::VisFlowOp;
use crate::services::persistence::AvailableServices as PAS;
use actix_web::{web, App, HttpServer};
use std::sync::Arc;

mod models;
mod server;
mod services;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //Create the services
    let vis_flow_log_repo = services::persistence::api::services::vis_flow_log::new().await;
    let vis_flow_op_repo = services::persistence::api::services::vis_flow_op::new().await;

    let app_state = AppState {
        services: AS {
            persistence: PAS {
                vis_flow_log: Arc::new(vis_flow_log_repo),
                vis_flow_op: Arc::new(vis_flow_op_repo),
            },
        },
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(server::route::save_logs)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

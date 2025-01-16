mod core;

use crate::core::persistence::connect_database;
use crate::core::web::api::models::AppState;
use core::web::start_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_pool = connect_database().await;
    let app_state = AppState { pool: db_pool };
    start_server(app_state).await
}

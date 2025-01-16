pub mod api;
mod application;
use crate::core::web::api::models::AppState;
use crate::core::web::application::routes::index;
use actix_web::{web, App, HttpServer};

pub async fn start_server(app_state: AppState) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(web::scope("/api/v1").service(index))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

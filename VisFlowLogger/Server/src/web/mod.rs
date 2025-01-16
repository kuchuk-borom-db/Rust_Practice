use actix_web::{web, App, HttpServer};
use application::routes::index;

mod application;

pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api/v1").service(index), //INSTRUCTION: Add other routes here
        )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

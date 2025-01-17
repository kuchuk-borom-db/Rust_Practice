mod database;
mod routes;
mod services;
use actix_web::{App, HttpServer};
use routes::index;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
/*
Services with API
and its implementation will use shared db pool
 */

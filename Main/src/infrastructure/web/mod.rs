pub(super) mod internal;

use actix_web::{App, HttpServer};
use internal::routes::*;
pub async fn run_server() -> std::io::Result<()> {
  let bind_address = "127.0.0.1:8080";

  println!("Server is running at http://{}", bind_address);

  HttpServer::new(|| App::new().service(handle_log_entries))
    .bind(bind_address)?
    .run()
    .await
}

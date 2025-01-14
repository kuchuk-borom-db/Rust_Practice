mod domain;
mod infrastructure;
use crate::infrastructure::web::run_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run_server().await
}

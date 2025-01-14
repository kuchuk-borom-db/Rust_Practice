mod handler;

use crate::infrastructure::web::internal::payloads::VisLogEntry;
use actix_web::{post, web, HttpResponse, Responder};

#[post("/")]
async fn handle_log_entries(req: web::Json<Vec<VisLogEntry>>) -> impl Responder {
    let result = handler::generate_diagram_from_vis_log_entries(req.into_inner());
    HttpResponse::Ok().body(result.unwrap())
}

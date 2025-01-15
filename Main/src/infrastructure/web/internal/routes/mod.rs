mod handler;

use crate::infrastructure::web::internal::payloads::VisLogEntry;
use actix_web::{post, web, HttpResponse, Responder};
use base64::{engine::general_purpose::STANDARD, Engine};

#[post("/")]
async fn handle_log_entries(req: web::Json<Vec<VisLogEntry>>) -> impl Responder {
    let result = handler::generate_diagram_from_vis_log_entries(req.into_inner());
    println!("{}", result.as_ref().unwrap());
    let encoded = STANDARD.encode(result.as_ref().unwrap());
    HttpResponse::Ok().body(encoded)
}
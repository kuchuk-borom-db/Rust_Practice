use actix_web::{get, HttpResponse};

mod handlers;

#[get("/")]
pub fn index() -> HttpResponse {
    HttpResponse::Ok().body("<h1>Hi from Rust</h1>")
}

use actix_web::{get, HttpResponse};
#[get("/")]
async fn index() -> HttpResponse {
  HttpResponse::Ok().body("Hello, World!")
}
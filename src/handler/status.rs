use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn status() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

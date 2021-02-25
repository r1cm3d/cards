use actix_web::{HttpResponse, Responder};

pub async fn check_status() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

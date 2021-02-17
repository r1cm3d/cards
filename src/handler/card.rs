use crate::domain::card;
use crate::dto;
use actix_web::{post, web, HttpResponse, Responder};

pub static SCOPE: &str = "/cards";

#[post("")]
pub async fn create(
    _: web::Data<Box<dyn card::Creator>>,
    payload: web::Json<dto::Card>,
) -> impl Responder {
    let dto: dto::Card = payload.into_inner();
    println!("DTO: {}", dto);

    HttpResponse::Ok().body("OK")
}

//TODO: add unit test to cover Ok and Error scenarios
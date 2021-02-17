use crate::domain::card;
use crate::handler;
use actix_web::web;

pub fn default(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(handler::card::SCOPE)
            .data::<Box<dyn card::Creator>>(Box::new(card::Service::new()))
            .service(handler::card::create),
    )
    .service(handler::status::check_status);
}

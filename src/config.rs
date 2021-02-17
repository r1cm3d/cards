use crate::handler;
use actix_web::web;

pub fn default(cfg: &mut web::ServiceConfig) {
    cfg.service(handler::status::status);
}

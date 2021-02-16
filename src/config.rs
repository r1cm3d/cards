use actix_web::web;
use crate::handler;

pub fn default(cfg: &mut web::ServiceConfig) {
    cfg.service(handler::status::status);
}
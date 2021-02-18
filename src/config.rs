use crate::domain::card;
use crate::handler;
use actix_web::web;

pub fn default(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(handler::card::SCOPE)
            .data::<Box<dyn card::Creator>>(Box::new(card::Service::new()))
            .route("", web::post().to(handler::card::create)),
    )
    .route("/status", web::get().to(handler::status::check_status));
}

#[cfg(test)]
mod tests {
    use crate::config;
    use actix_service::Service;
    use actix_web::http::StatusCode;
    use actix_web::{test, App};

    #[actix_rt::test]
    async fn test_status_ok() {
        let mut app = test::init_service(App::new().configure(config::default)).await;
        let req = test::TestRequest::get().uri("/status").to_request();

        let resp = app.call(req).await.unwrap();
        let status = resp.status();
        let body = test::read_body(resp).await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "OK");
    }
}
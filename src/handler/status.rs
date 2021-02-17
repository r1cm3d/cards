use actix_web::{get, HttpResponse, Responder};

#[get("/status")]
pub async fn check_status() -> impl Responder {
    HttpResponse::Ok().body("OK")
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

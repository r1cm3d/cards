use actix_web::{get, App, HttpResponse, HttpServer, Responder};

static ADDRESS: &str = "127.0.0.1";
static PORT: &str = "8080";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(status)
    })
        .bind(format!("{}:{}", ADDRESS, PORT))?
        .run()
        .await
}

#[get("/")]
async fn status() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

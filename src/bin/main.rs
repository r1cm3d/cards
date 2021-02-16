use actix_web::{App, HttpServer};
use cards::handler;

static ADDRESS: &str = "127.0.0.1";
static PORT: &str = "8080";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(handler::status::status)
    })
        .bind(format!("{}:{}", ADDRESS, PORT))?
        .run()
        .await
}
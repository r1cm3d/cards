use actix_web::{App, HttpServer};

static ADDRESS: &str = "127.0.0.1";
static PORT: &str = "8080";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(cards::config::default))
        .bind(format!("{}:{}", ADDRESS, PORT))?
        .run()
        .await
}

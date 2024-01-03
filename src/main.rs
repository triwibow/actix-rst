mod handler;
mod model;
mod request;
mod response;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(handler::all_handler())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
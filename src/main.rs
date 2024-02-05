use actix_web::{get, post, web, App, Responder, HttpServer, HttpResponse};
use dotenv::dotenv;

mod databases;

use crate::databases::connection;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
    }).bind(("localhost", 9000)).unwrap().run().await
}

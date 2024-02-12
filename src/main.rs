use actix_web::{web, App, HttpServer};
use sqlx::{Pool, Postgres};
use dotenv::dotenv;

mod http;
mod app;

use crate::http::databases::connection::connection;
use crate::http::routes::routes;

pub struct AppState {
    pool: Pool<Postgres>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = connection().await;
    
    println!("Server iniciado com sucesso! 🦀");

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(AppState { pool: pool.clone() }))
        .configure(&routes)
        // .wrap() // middlewares
    }).bind(("localhost", 9000)).unwrap().run().await
}

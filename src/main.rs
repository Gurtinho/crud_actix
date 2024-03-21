use std::env::*;

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{web, App, HttpServer};
use sqlx::{Pool, Postgres};
use dotenv::dotenv;

mod http;
mod app;

use crate::http::databases::connection::connection;
use crate::http::routes::config;

pub struct AppState {
    pool: Pool<Postgres>,
    jwt: String
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let jwt_env = var("JSON_WEB_TOKEN_SECRET").unwrap_or_else(|c| format!("JWT error {}", c));

    let port = var("WEB_SERVER_PORT").unwrap().to_owned().parse().unwrap();

    let pool = connection().await;

    println!("Server iniciado com sucesso! 🦀");
    HttpServer::new(move || {
        let cors = Cors::default()
        // .allowed_origin("http://localhost:3000")
        .allow_any_origin()
        .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE"])
        .allowed_headers(vec![
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCEPT,
        ])
        .supports_credentials();

        App::new()
        .app_data(web::Data::new(AppState {
            pool: pool.clone(), 
            jwt: jwt_env.clone() 
        }))
        .configure(&config)
        .wrap(cors)
        // .wrap() // middlewares
    }).bind(("0.0.0.0", port)).unwrap().run().await
}

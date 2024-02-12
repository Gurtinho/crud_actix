use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env::*;

pub async fn connection() -> Pool<Postgres> {
  let database_url = var("DATABASE_URL").expect("DATABASE_URL must be set");
  let pool = match PgPoolOptions::new()
    .max_connections(10)
    .connect(&database_url)
    .await
  {
    Ok(pool) => {
      println!("Conectado com banco de dados 🔥");
      pool
    }
    Err(_) => {
      println!("Falha ao conectar com banco de dados 🧊");
      std::process::exit(1);
    }
  };
  pool
}
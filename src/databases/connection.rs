use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};
use std::env::*;

pub async fn connection() -> Pool<Postgres> {
  let database_url = var("DATABASE_URL").expect("DATABASE_URL must be set");
  let pool = match PgPoolOptions::new()
    .max_connections(10)
    .connect(&database_url)
    .await
  {
    Ok(pool) => {
      println!("Conectado com o banco de dados! 🔥");
      pool
    }
    Err(_) => {
      println!("Falha ao se conectar com banco de dados 🧊");
      std::process::exit(1);
    }
  };
  pool
}
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct ParamTicketSchema {
  pub id: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateUserSchema {
  pub username: String,
  pub email: String,
  pub password: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateUserSchema {
  pub username: String,
  pub email: String,
  pub password: String,
  pub updated_at: chrono::NaiveDateTime
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetUserSchema {
  pub id: Uuid,
  pub username: String,
  pub email: String,
  pub password: String,
  pub created_at: chrono::NaiveDateTime,
  pub updated_at: chrono::NaiveDateTime
}
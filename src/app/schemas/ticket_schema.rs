use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ParamTicketSchema {
  pub id: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateTicketSchema {
  pub subject: String,
  pub description: Option<String>,
  pub status: i32,
  pub priority: i32
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateTicketSchema {
  pub subject: String,
  pub description: Option<String>,
  pub status: i32,
  pub priority: i32,
  pub updated_at: Option<chrono::NaiveDateTime>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetTicketSchema {
  pub id: String,
  pub subject: String,
  pub description: Option<String>,
  pub status: i32,
  pub priority: i32,
  pub created_at: Option<chrono::NaiveDateTime>,
  pub updated_at: Option<chrono::NaiveDateTime>
}
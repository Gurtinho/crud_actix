use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Ticket {
  pub id: Uuid,
  pub subject: String,
  pub description: Option<String>,
  pub status: i32, // 0 = FECHADO, 1 = ABERTO, 2 = ANDAMENTO, 3 = CONCLUIDO
  pub priority: i32, // 0 = NENHUMA, 1 = BAIXA, 2 = NORMAL, 3 = ALTA, 4 = URGENTE
  pub created_at: Option<chrono::NaiveDateTime>,
  pub updated_at: Option<chrono::NaiveDateTime>
}
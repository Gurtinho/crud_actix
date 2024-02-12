use actix_web::{get, patch, post, delete, web, HttpResponse, Responder};
use chrono::Utc;
use uuid::Uuid;

use crate::{
  app::{
    models::ticket_model::Ticket,
    schemas::ticket_schema::{CreateTicketSchema, UpdateTicketSchema}
  }, http::utils::date_now, AppState
};
use crate::http::view_model::{http_error, http_response};

#[post("/ticket")]
pub async fn create_ticket(
  body: web::Json<CreateTicketSchema>,
  data: web::Data<AppState>
  ) -> impl Responder {

  let mut description = String::new();
  if let Some(desc) = &body.description {
    description = desc.to_string();
  }
 
  let result = sqlx::query_as!(
    Ticket,
    "INSERT INTO tickets (id, subject, description, status, priority) VALUES ($1, $2, $3, $4, $5) RETURNING *",
    Uuid::new_v4(),
    body.subject.to_string(),
    description,
    body.status,
    body.priority
  ).fetch_one(&data.pool)
  .await;

  match result {
    Ok(ticket) => {
      return http_response(HttpResponse::Ok(), serde_json::json!(&ticket));
    }
    Err(e) => {
      return http_error(HttpResponse::InternalServerError(), format!("{:?}", &e));
    }
  }
}

#[get("/tickets")]
pub async fn get_tickets(
  data: web::Data<AppState>
) -> impl Responder {

  let result = sqlx::query_as!(
    Ticket,
    "SELECT * FROM tickets;"
  )
  .fetch_all(&data.pool)
  .await;

  if result.is_err() {
    return http_error(
      HttpResponse::InternalServerError(),
      "Um erro ocorreu ao consultar os tickets"
    );
  }

  http_response(HttpResponse::Ok(), serde_json::json!(&result.unwrap()))
}

#[get("/tickets/{id}")]
pub async fn get_ticket(
  path: web::Path<uuid::Uuid>,
  data: web::Data<AppState>
) -> impl Responder {
  let result = sqlx::query_as!(
    Ticket,
    "SELECT * FROM tickets WHERE id = $1",
    &path.into_inner()
  )
  .fetch_one(&data.pool)
  .await;

  match result {
    Ok(ticket) => {
      return http_response(HttpResponse::Ok(), serde_json::json!(&ticket))
    }
    Err(e) => {
      return http_error(HttpResponse::InternalServerError(), format!("{:?}", &e))
    }
  }
}

#[patch("/ticket/{id}")]
pub async fn update_ticket(
  path: web::Path<uuid::Uuid>,
  body: web::Json<UpdateTicketSchema>,
  data: web::Data<AppState>
) -> impl Responder {
  let ticket_id = &path.into_inner();
  let ticket_found = sqlx::query_as!(
    Ticket,
    "SELECT * FROM tickets WHERE id = $1",
    &ticket_id
  ).fetch_one(&data.pool).await;

  if ticket_found.is_err() {
    return http_error(
     HttpResponse::InternalServerError(),
    format!("Id não encontrado: {}", &ticket_id)
    );
  }

  let result = sqlx::query_as!(
    Ticket,
    "UPDATE tickets SET subject = $1, description = $2, status = $3, priority = $4, updated_at = $5 WHERE id = $6 RETURNING *",
    body.subject.to_owned(),
    body.description.to_owned(),
    body.status.to_owned(),
    body.priority.to_owned(),
    date_now(),
    ticket_id
  ).fetch_one(&data.pool).await;

  match result {
    Ok(ticket) => {
      return http_response(HttpResponse::Ok(), serde_json::json!(&ticket));
    }
    Err(e) => {
      return http_error(HttpResponse::InternalServerError(), format!("{:?}", &e))
    }
  }
}
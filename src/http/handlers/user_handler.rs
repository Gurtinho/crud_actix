use actix_web::{web, HttpResponse, Responder};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use uuid::Uuid;

use crate::{app::{models::user_model::User, schemas::user_schema::{
  CreateUserSchema, JWTClaimsSchema, LoginUserSchema, UpdateUserSchema
}}, http::view_model::{http_error, http_response}, AppState};

pub async fn create_user(
  body: web::Json<CreateUserSchema>,
  data: web::Data<AppState>
) -> impl Responder {
  let user_founded = sqlx::query_as!(
    User,
    "SELECT * FROM users WHERE email = $1",
    &body.email.to_string()
  )
  .fetch_one(&data.pool).await;

  match user_founded {
    Ok(_) => {
      return http_error(
        HttpResponse::InternalServerError(),
        format!("User already exists")
      );
    }
    Err(_) => {}
  }

  let salt = SaltString::generate(&mut OsRng);
  let pass_hash = Argon2::default()
  .hash_password(&body.password.as_bytes(), &salt).unwrap().to_string();

  let result = sqlx::query_as!(
    User,
    "INSERT INTO users (id, username, email, password) VALUES ($1, $2, $3, $4) RETURNING *",
    Uuid::new_v4(),
    body.username.to_string(),
    body.email.to_string(),
    pass_hash
  ).fetch_one(&data.pool)
  .await;

  match result {
    Ok(user) => {
      return http_response(
       HttpResponse::Ok(), 
        serde_json::json!(user)
      )
    }
    Err(e) => {
    return http_error(
       HttpResponse::InternalServerError(), 
      format!("Error {}", e)
      )
    }
  }
}

pub async fn update_user(
  path: web::Path<uuid::Uuid>,
  body: web::Json<UpdateUserSchema>,
  data: web::Data<AppState>
) -> impl Responder {
  let user_id = &path.into_inner();

  let user_founded = sqlx::query_as!(
    User,
    "SELECT * FROM users WHERE id = $1",
    &user_id
  )
  .fetch_one(&data.pool).await;

  if user_founded.is_err() {
    return http_error(
      HttpResponse::InternalServerError(),
      format!("Id not found: {}", &user_id)
    );
  }

  let user_founded = user_founded.unwrap();

  let parsed_hash = PasswordHash::new(&user_founded.password).unwrap();
  let parsed_hash = Argon2::default().verify_password(&body.password.as_bytes(), &parsed_hash).is_ok();

  if !parsed_hash {
    return http_error(
      HttpResponse::InternalServerError(),
      format!("Password dont match")
    );
  }

  let result = sqlx::query_as!(
    User,
    "UPDATE users SET username = $1, email = $2 WHERE id = $3 RETURNING *",
    body.username.to_string(),
    body.email.to_string(),
    &user_id
  ).fetch_one(&data.pool).await;

  match result {
    Ok(ok) => {
      return http_response(
        HttpResponse::Ok(), 
        serde_json::json!(ok)
      )
    }
    Err(e) => {
      return http_error(
        HttpResponse::InternalServerError(),
        format!("Error: {}", &e)
      );
    }
  }
}

pub async fn get_user(
  path: web::Path<uuid::Uuid>,
  data: web::Data<AppState>
) -> impl Responder {
  let user_id = &path.into_inner();

  let user_founded = sqlx::query_as!(
    User,
    "SELECT * FROM users WHERE id = $1",
    &user_id
  )
  .fetch_one(&data.pool).await;

  if user_founded.is_err() {
    return http_error(
      HttpResponse::InternalServerError(),
      format!("Id not found: {}", &user_id)
    );
  }

  let user_founded = user_founded.unwrap();

  return http_response(
    HttpResponse::Ok(), 
    serde_json::json!(user_founded)
  )
}

pub async fn login_user(
  body: web::Json<LoginUserSchema>,
  data: web::Data<AppState>
) -> impl Responder {
  return http_response(
    HttpResponse::Ok(),
    serde_json::json!("aaa")
  )
}
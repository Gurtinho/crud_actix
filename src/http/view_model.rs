use actix_web::{HttpResponse, HttpResponseBuilder};
use serde_json::Value;

pub fn http_response(mut status: HttpResponseBuilder, json: Value) -> HttpResponse {
  status.json(serde_json::json!({
    "status": "success",
    "data": &json
  }))
}

// tipagem genérica para strings
pub fn http_error<T: AsRef<str>>(mut status: HttpResponseBuilder, message: T) -> HttpResponse {
  status.json(serde_json::json!({
    "status": "error",
    "message": &message.as_ref()
  }))
}
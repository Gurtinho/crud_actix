use actix_web::web::{delete, get, patch, post, scope, ServiceConfig};

use crate::http::handlers::*;

pub fn config(conf: &mut ServiceConfig) {
  // Tickets
  conf.service(
    scope("tickets")
    .route("", post().to(ticket_handler::create_ticket))
    .route("", get().to(ticket_handler::get_tickets))
    .route("{id}", get().to(ticket_handler::get_ticket))
    .route("{id}", patch().to(ticket_handler::update_ticket))
    .route("{id}", delete().to(ticket_handler::delete_ticket))
  );

  // Users
  conf.service(
    scope("users")
      .route("", post().to(user_handler::create_user))
      .route("{id}", patch().to(user_handler::update_user))
      .route("{id}", get().to(user_handler::get_user))
      // .route("login", delete().to(user_handler::login_user))
  );
}
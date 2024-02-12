use actix_web::web;

use crate::http::handlers::*;

pub fn routes(conf: &mut web::ServiceConfig) {
  let ticket = web::scope("")
    .service(ticket_handler::create_ticket)
    .service(ticket_handler::get_tickets)
    .service(ticket_handler::get_ticket)
    .service(ticket_handler::update_ticket);

  // let user = web::scope("")
  //   .service(user_handler::)

  conf.service(ticket);
}
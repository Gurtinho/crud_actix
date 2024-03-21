use actix_web::Responder;

pub trait TicketInterface {
  fn create_ticket() -> impl Responder;
  fn get_tickets() -> impl Responder;
}
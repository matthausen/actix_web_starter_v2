use actix_web::web;
use crate::api::handler_create_item::create_item_handler;

pub fn configure_routes(cfg: &mut actix_web::web::ServiceConfig) {
    // Define your routes and route handlers here
    // Example:
    cfg.service(web::resource("/items").route(web::post().to(create_item_handler)));
    // Add other routes and handlers as needed
}
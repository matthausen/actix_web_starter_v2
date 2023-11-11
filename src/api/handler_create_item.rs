use actix_web::web;
use std::sync::Arc;
use serde_derive::{Deserialize, Serialize };
use crate::services::item::storage::service::ItemStorage;
use crate::services::item::storage::model::Item;



#[derive(Debug, Deserialize, Serialize)]
pub struct ItemRequest {
    id: Option<String>,
    name: Option<String>,
    // Add other fields as needed
}


// Example route handler
pub async fn create_item_handler(
    item: web::Json<ItemRequest>, 
    item_storage: web::Data<Arc<dyn ItemStorage>>,
) -> impl actix_web::Responder {

    // logic
    println!("handler started");

    let input = item_req_to_item_model(item);

    match item_storage.create_item(input.into_inner()).await {
        Ok(_) => actix_web::HttpResponse::Ok().finish(),
        Err(e) => {
            println!("handler error {}", e);
            actix_web::HttpResponse::InternalServerError().finish()
        },
    }
}

fn item_req_to_item_model(req: web::Json<ItemRequest>) -> web::Json<Item> {
    // Extract the fields from the request JSON
    let id = req.id.clone();
    let name = req.name.clone();

    // Create a new Item object
    let item = Item { id, name /*, add other fields as needed */ };

    // Convert Item object to JSON and respond with it
    web::Json(item)
}
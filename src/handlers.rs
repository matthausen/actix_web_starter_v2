use actix_web::{web, HttpResponse};
use crate::Item;
use crate::services::ItemService;
use std::sync::Arc;

pub async fn create_item(item: web::Json<Item>, item_service: web::Data<Arc<ItemService>>) -> HttpResponse {
    match item_service.create_item(item.into_inner()).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

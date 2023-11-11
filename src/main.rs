use actix_web::{web, App, HttpServer};
use tokio::sync::Mutex;
use std::sync::Arc;
use rusoto_core::Region;
use rusoto_dynamodb::DynamoDbClient;

mod api;
mod services;

use crate::services::item::storage::service::{DynamoDbItemRepository, ItemStorage};

use crate::api::routes::configure_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = DynamoDbClient::new(Region::Custom {
        name: "local".to_string(),
        endpoint: "http://localhost:8000".to_string(),
    });
    let table_name = "items".to_string();

    // Wrap the client in an Arc and Mutex
    let client = Arc::new(Mutex::new(client));
   

    // Create a shared storage instance
    let item_storage: Arc<dyn ItemStorage> = Arc::new(DynamoDbItemRepository { client, table_name });


    // Start the Actix web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(item_storage.clone()))
            .configure(configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

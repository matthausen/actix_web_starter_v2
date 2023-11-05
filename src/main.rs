use actix_web::{web, App, HttpServer};
use serde_derive::Serialize;
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::repositories::DynamoDbItemRepository;
use rusoto_dynamodb::DynamoDbClient;

mod handlers;
mod services;
mod repositories;

#[derive(Debug, serde::Deserialize, Serialize)]
pub struct Item {
    id: Option<String>,
    name: Option<String>,
    // Add other fields as needed
}

#[async_trait::async_trait]
trait ItemRepository: Send + Sync {
    async fn create_item(&self, item: Item) -> Result<(), Box<dyn std::error::Error>>;
    // Add other repository methods as needed
}

#[async_trait::async_trait]
impl ItemRepository for DynamoDbItemRepository {
    async fn create_item(&self, item: Item) -> Result<(), Box<dyn std::error::Error>> {
        // Implement the logic to create an item in DynamoDB here
        Ok(())
    }
    // Implement other repository methods as needed
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = DynamoDbClient::new(Default::default());
    let table_name = "your_table_name".to_string();

    // Wrap the client in an Arc and Mutex
    let client = Arc::new(Mutex::new(client));

    // Create a shared repository instance
    let item_repository: Arc<dyn ItemRepository> = Arc::new(DynamoDbItemRepository { client, table_name });


    // Start the Actix web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(item_repository.clone()))
            .configure(configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn configure_routes(cfg: &mut actix_web::web::ServiceConfig) {
    // Define your routes and route handlers here
    // Example:
    cfg.service(web::resource("/items").route(web::post().to(create_item_handler)));
    // Add other routes and handlers as needed
}

// Example route handler
async fn create_item_handler(
    item: web::Json<Item>, 
    item_repository: web::Data<Arc<dyn ItemRepository>>,
) -> impl actix_web::Responder {
    match item_repository.create_item(item.into_inner()).await {
        Ok(_) => actix_web::HttpResponse::Ok().finish(),
        Err(_) => actix_web::HttpResponse::InternalServerError().finish(),
    }
}

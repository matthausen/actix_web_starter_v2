use crate::Item;
use rusoto_dynamodb::{AttributeValue, DynamoDb, PutItemInput};
use std::collections::HashMap;
use tokio::sync::Mutex;
use std::sync::Arc;
use async_trait::async_trait;

#[async_trait]
pub trait ItemRepository: Send + Sync {
    async fn create_item(&self, item: Item) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct DynamoDbItemRepository {
    pub client: Arc<Mutex<rusoto_dynamodb::DynamoDbClient>>,
    pub table_name: String,
}

impl DynamoDbItemRepository {
    fn convert_to_attribute_values(item: &Item) -> HashMap<String, AttributeValue> {
        let mut attribute_values = HashMap::new();

        if let Some(id) = &item.id {
            let id_attribute_value = AttributeValue {
                s: Some(id.clone()),
                ..Default::default()
            };
            attribute_values.insert("id".to_string(), id_attribute_value);
        }
    
        if let Some(name) = &item.name {
            let name_attribute_value = AttributeValue {
                s: Some(name.clone()),
                ..Default::default()
            };
            attribute_values.insert("name".to_string(), name_attribute_value);
        }
    
        // Add other fields as needed
        // Example:
        // if let Some(field_value) = &item.field_name {
        //     let field_name_attribute_value = AttributeValue {
        //         s: Some(field_value.clone()),
        //         ..Default::default()
        //     };
        //     attribute_values.insert("field_name".to_string(), field_name_attribute_value);
        // }
    
        attribute_values
    }
}

#[async_trait]
impl ItemRepository for DynamoDbItemRepository {
    async fn create_item(&self, item: Item) -> Result<(), Box<dyn std::error::Error>> {
        let input_values = DynamoDbItemRepository::convert_to_attribute_values(&item);
        let input = PutItemInput {
            table_name: self.table_name.clone(),
            item: input_values,
            ..Default::default()
        };

        let mut client = self.client.lock().await;
        client.put_item(input).await?;
        Ok(())
    }
}

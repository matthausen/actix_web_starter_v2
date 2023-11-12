use rusoto_dynamodb::{AttributeValue, DynamoDb, GetItemInput, PutItemInput};
use std::collections::HashMap;
use tokio::sync::Mutex;
use std::sync::Arc;
use async_trait::async_trait;

use crate::services::item::storage::model::Item;

#[async_trait]
pub trait ItemStorage: Send + Sync {
    async fn create_item(&self, item: Item) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_item(&self, key_name: &str, key_value: &str) -> Result<Option<Item>, Box<dyn std::error::Error>>;
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
    
        attribute_values
    }

    fn convert_to_item(attribute_values: HashMap<String, AttributeValue>) -> Item {
        let result_item = Item {
            id: attribute_values.get("id").and_then(|value| value.s.clone()),
            name: attribute_values.get("name").and_then(|value| value.s.clone()),
        };

        result_item
    }
}

#[async_trait]
impl ItemStorage for DynamoDbItemRepository {
    // Create Item
    async fn create_item(&self, item: Item) -> Result<(), Box<dyn std::error::Error>> {
        let input_values = DynamoDbItemRepository::convert_to_attribute_values(&item);
        let input = PutItemInput {
            table_name: self.table_name.clone(),
            item: input_values,
            ..Default::default()
        };

        let client = self.client.lock().await;
        client.put_item(input).await?;
        Ok(())
    }


    // Get Item
    async fn get_item(&self, key_name: &str, key_value: &str) -> Result<Option<Item>, Box<dyn std::error::Error>> {
        let client = self.client.lock().await;

        let input = GetItemInput {
            table_name: self.table_name.clone(),
            key: {
                let mut key = HashMap::new();
                key.insert(key_name.to_string(), AttributeValue {
                    s: Some(key_value.to_string()),
                    ..Default::default()
                });
                key
            },
            ..Default::default()
        };

        match client.get_item(input).await {
            Ok(response) => {
                if let Some(item) = response.item {
                    // Convert DynamoDB AttributeValues back to your Item struct
                    let result_item = DynamoDbItemRepository::convert_to_item(item);
                    Ok(Some(result_item))
                } else {
                    Ok(None)
                }
            }
            Err(err) => Err(Box::new(err)),
        }
    }
}

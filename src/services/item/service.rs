use std::sync::Arc;
use crate::Item;
use crate::storage::ItemStorage;

#[derive(Clone)]
pub struct ItemService {
    pub storage: Arc<dyn ItemStorage>,
}

impl ItemService {
    pub fn new(storage: Arc<dyn ItemStorage>) -> Self {
        ItemService { storage }
    }

    pub async fn create_item(&self, item: Item) -> Result<(), Box<dyn std::error::Error>> {
        self.storage.create_item(item).await
    }
}

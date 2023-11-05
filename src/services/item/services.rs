use std::sync::Arc;
use crate::Item;
use crate::repositories::ItemStorage;

#[derive(Clone)]
pub struct ItemService {
    pub repository: Arc<dyn ItemStorage>,
}

impl ItemService {
    pub fn new(repository: Arc<dyn ItemStorage>) -> Self {
        ItemService { repository }
    }

    pub async fn create_item(&self, item: Item) -> Result<(), Box<dyn std::error::Error>> {
        self.repository.create_item(item).await
    }
}

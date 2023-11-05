use std::sync::Arc;
use crate::Item;
use crate::repositories::ItemRepository;

#[derive(Clone)]
pub struct ItemService {
    pub repository: Arc<dyn ItemRepository>,
}

impl ItemService {
    pub fn new(repository: Arc<dyn ItemRepository>) -> Self {
        ItemService { repository }
    }

    pub async fn create_item(&self, item: Item) -> Result<(), Box<dyn std::error::Error>> {
        self.repository.create_item(item).await
    }
}

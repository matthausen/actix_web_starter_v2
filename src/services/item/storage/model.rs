use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    pub id: Option<String>,
    pub name: Option<String>,
    // Add other fields as needed
}
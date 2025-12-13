use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Webhook {
    pub id: String,
    pub name: String,
    pub url: String,
    pub created_at: String,
}

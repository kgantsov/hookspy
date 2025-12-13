use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateWebhookRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct WebhookRequest {
    pub id: String,
    pub webhook_id: String,
    pub method: String,
    pub headers: String,
    pub body: String,
    pub received_at: String,
}

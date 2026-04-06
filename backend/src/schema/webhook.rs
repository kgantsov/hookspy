use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateWebhookRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct WebhookRequest {
    pub id: String,
    pub webhook_id: String,
    pub method: String,
    pub headers: String,
    pub body: String,
    pub received_at: String,
    pub caller_ip: Option<String>,
    pub duration_us: Option<u64>,
}

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserWebhookStats {
    pub email: String,
    pub count: u64,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Stats {
    pub total_users: u64,
    pub total_webhooks: u64,
    pub total_requests_received: u64,
    pub webhooks_per_user: Vec<UserWebhookStats>,
}

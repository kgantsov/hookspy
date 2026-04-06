use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Webhook {
    pub id: String,
    pub name: String,
    pub url: String,
    pub created_at: String,
    pub last_seen_at: Option<String>,
    pub has_unread: bool,
}

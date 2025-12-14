use crate::notification::notification::Notification;

use libsql::Connection;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Connection>>,
    pub notification: Arc<Mutex<Notification>>,
    pub domain: String,
}

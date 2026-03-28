use crate::{config::Config, notification::notification::Notification};

use std::sync::Arc;
use tokio::sync::Mutex;
use turso::Connection;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Connection>>,
    pub notification: Arc<Mutex<Notification>>,
    pub domain: String,
    pub config: Config,
}

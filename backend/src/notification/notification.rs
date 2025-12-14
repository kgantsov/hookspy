use axum::extract::ws::Message;
use std::collections::HashMap;
use tokio::sync::mpsc;

pub struct Notification {
    pub subscribers: HashMap<String, Vec<(String, mpsc::Sender<Message>)>>,
}

impl Notification {
    pub fn new() -> Self {
        Self {
            subscribers: HashMap::new(),
        }
    }

    pub fn subscribe(&mut self, webhook_id: String, session_id: String, tx: mpsc::Sender<Message>) {
        self.subscribers
            .entry(webhook_id)
            .or_default()
            .push((session_id, tx));
    }

    pub fn unsubscribe(&mut self, session_id: &str) {
        self.subscribers.retain(|_, subs| {
            subs.retain(|(id, _)| id != session_id);
            !subs.is_empty()
        });
    }

    pub async fn notify(&mut self, webhook_id: String, message: String) {
        if let Some(subs) = self.subscribers.get_mut(&webhook_id) {
            subs.retain(|(_, tx)| tx.try_send(Message::Text(message.clone())).is_ok());
        }
    }
}

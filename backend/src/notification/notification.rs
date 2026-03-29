use axum::extract::ws::Message;
use std::collections::HashMap;
use tokio::sync::mpsc;

pub struct Notification {
    /// Per-webhook subscribers: webhook_id -> [(session_id, tx)]
    pub subscribers: HashMap<String, Vec<(String, mpsc::Sender<Message>)>>,
    /// Per-user subscribers: user_id -> [(session_id, tx)]
    pub user_subscribers: HashMap<String, Vec<(String, mpsc::Sender<Message>)>>,
}

impl Notification {
    pub fn new() -> Self {
        Self {
            subscribers: HashMap::new(),
            user_subscribers: HashMap::new(),
        }
    }

    pub fn subscribe(&mut self, webhook_id: String, session_id: String, tx: mpsc::Sender<Message>) {
        self.subscribers
            .entry(webhook_id)
            .or_default()
            .push((session_id, tx));
    }

    pub fn subscribe_user(
        &mut self,
        user_id: String,
        session_id: String,
        tx: mpsc::Sender<Message>,
    ) {
        self.user_subscribers
            .entry(user_id)
            .or_default()
            .push((session_id, tx));
    }

    pub fn unsubscribe(&mut self, session_id: &str) {
        self.subscribers.retain(|_, subs| {
            subs.retain(|(id, _)| id != session_id);
            !subs.is_empty()
        });
        self.user_subscribers.retain(|_, subs| {
            subs.retain(|(id, _)| id != session_id);
            !subs.is_empty()
        });
    }

    pub async fn notify(&mut self, webhook_id: String, message: String) {
        if let Some(subs) = self.subscribers.get_mut(&webhook_id) {
            subs.retain(|(_, tx)| tx.try_send(Message::Text(message.clone())).is_ok());
        }
    }

    /// Notify all per-user subscribers that a new request arrived for `webhook_id`.
    /// The message payload is just the webhook_id so the frontend knows which item to mark unread.
    pub async fn notify_user(&mut self, user_id: &str, webhook_id: &str) {
        if let Some(subs) = self.user_subscribers.get_mut(user_id) {
            let message = webhook_id.to_string();
            subs.retain(|(_, tx)| tx.try_send(Message::Text(message.clone())).is_ok());
        }
    }
}

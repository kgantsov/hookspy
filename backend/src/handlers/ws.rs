use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::app::AppState;

pub async fn webhook_notifications_ws(
    State(state): State<AppState>,
    Path(webhook_id): Path<String>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    println!(
        "Webhook notifications websocket requested for webhook ID: {}",
        webhook_id.clone()
    );
    ws.on_upgrade(move |socket| handle_socket(state, socket, webhook_id))
}

async fn handle_socket(state: AppState, socket: WebSocket, webhook_id: String) {
    let session_id = Uuid::new_v4().to_string();
    println!("Client connected: {}", session_id);

    let (mut ws_tx, mut ws_rx) = socket.split();
    let (tx, mut rx) = mpsc::channel::<Message>(32);

    // Register subscriber
    {
        let mut notification = state.notification.lock().await;
        notification.subscribe(webhook_id.clone(), session_id.clone(), tx);
    }

    // Task: send notifications → websocket
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if ws_tx.send(msg).await.is_err() {
                break;
            }
        }
    });

    // Task: receive messages from client
    while let Some(Ok(msg)) = ws_rx.next().await {
        match msg {
            Message::Text(text) => {
                println!("{} -> {}", session_id, text);
            }
            Message::Close(_) => break,
            _ => {}
        }
    }

    // Cleanup
    {
        let mut notification = state.notification.lock().await;
        notification.unsubscribe(&session_id);
    }

    send_task.abort();
    println!("Client disconnected: {}", session_id);
}

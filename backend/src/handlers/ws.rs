use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    http::StatusCode,
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use tracing::info;
use uuid::Uuid;

use crate::app::AppState;
use crate::auth::jwt::AuthUser;

pub async fn webhook_notifications_ws(
    State(state): State<AppState>,
    Path(webhook_id): Path<String>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    info!("Client connecting to webhook ID: {}", webhook_id.clone());
    ws.on_upgrade(move |socket| handle_socket(state, socket, webhook_id))
}

pub async fn user_notifications_ws(
    State(state): State<AppState>,
    auth_user: Result<AuthUser, (StatusCode, &'static str)>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    match auth_user {
        Ok(AuthUser(claims)) => {
            let user_id = claims.sub.clone();
            info!("User {} connecting to user-level notifications", user_id);
            ws.on_upgrade(move |socket| handle_user_socket(state, socket, user_id))
                .into_response()
        }
        Err((status, msg)) => (status, msg).into_response(),
    }
}

async fn handle_socket(state: AppState, socket: WebSocket, webhook_id: String) {
    let session_id = Uuid::new_v4().to_string();
    info!("Client connected: {} {}", webhook_id, session_id);

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
                info!("{} {} -> {}", webhook_id, session_id, text);
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
    info!("Client disconnected: {} {}", webhook_id, session_id);
}

async fn handle_user_socket(state: AppState, socket: WebSocket, user_id: String) {
    let session_id = Uuid::new_v4().to_string();
    info!("User WS connected: {} {}", user_id, session_id);

    let (mut ws_tx, mut ws_rx) = socket.split();
    let (tx, mut rx) = mpsc::channel::<Message>(32);

    // Register per-user subscriber
    {
        let mut notification = state.notification.lock().await;
        notification.subscribe_user(user_id.clone(), session_id.clone(), tx);
    }

    // Task: forward notifications to websocket
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if ws_tx.send(msg).await.is_err() {
                break;
            }
        }
    });

    // Drain incoming messages (keepalive / close)
    while let Some(Ok(msg)) = ws_rx.next().await {
        if let Message::Close(_) = msg {
            break;
        }
    }

    // Cleanup
    {
        let mut notification = state.notification.lock().await;
        notification.unsubscribe(&session_id);
    }

    send_task.abort();
    info!("User WS disconnected: {} {}", user_id, session_id);
}

use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Json,
    routing::{get, post},
    Router,
};
use libsql::{Builder, Connection};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    db: Arc<Mutex<Connection>>,
}

#[derive(Serialize, Deserialize)]
struct Webhook {
    id: String,
    name: String,
    created_at: String,
}

#[derive(Serialize, Deserialize)]
struct CreateWebhookRequest {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct WebhookRequest {
    id: String,
    webhook_id: String,
    method: String,
    headers: String,
    body: String,
    received_at: String,
}

async fn init_db(conn: &Connection) -> Result<(), libsql::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS webhooks (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            created_at TEXT NOT NULL
        )",
        (),
    )
    .await?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS webhook_requests (
            id TEXT PRIMARY KEY,
            webhook_id TEXT NOT NULL,
            method TEXT NOT NULL,
            headers TEXT NOT NULL,
            body TEXT NOT NULL,
            received_at TEXT NOT NULL,
            FOREIGN KEY (webhook_id) REFERENCES webhooks(id)
        )",
        (),
    )
    .await?;

    Ok(())
}

async fn create_webhook(
    State(state): State<AppState>,
    Json(payload): Json<CreateWebhookRequest>,
) -> Result<Json<Webhook>, StatusCode> {
    let id = Uuid::new_v4().to_string();
    let created_at = chrono::Utc::now().to_rfc3339();

    let db = state.db.lock().await;
    db.execute(
        "INSERT INTO webhooks (id, name, created_at) VALUES (?, ?, ?)",
        libsql::params![id.clone(), payload.name.clone(), created_at.clone()],
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(Webhook {
        id,
        name: payload.name,
        created_at,
    }))
}

async fn list_webhooks(State(state): State<AppState>) -> Result<Json<Vec<Webhook>>, StatusCode> {
    let db = state.db.lock().await;
    let mut rows = db
        .query(
            "SELECT id, name, created_at FROM webhooks ORDER BY created_at DESC",
            (),
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut webhooks = Vec::new();
    while let Some(row) = rows
        .next()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        webhooks.push(Webhook {
            id: row.get(0).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
            name: row.get(1).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
            created_at: row.get(2).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        });
    }

    Ok(Json(webhooks))
}

async fn receive_webhook(
    State(state): State<AppState>,
    Path(webhook_id): Path<String>,
    headers: HeaderMap,
    body: String,
) -> Result<Json<WebhookRequest>, StatusCode> {
    let id = Uuid::new_v4().to_string();
    let received_at = chrono::Utc::now().to_rfc3339();

    // Convert HeaderMap to a serializable HashMap
    let headers_map: std::collections::HashMap<String, String> = headers
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect();
    let headers_json = serde_json::to_string(&headers_map).unwrap_or_else(|_| "{}".to_string());

    let db = state.db.lock().await;

    // Verify webhook exists
    let mut rows = db
        .query(
            "SELECT id FROM webhooks WHERE id = ?",
            libsql::params![webhook_id.clone()],
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if rows
        .next()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .is_none()
    {
        return Err(StatusCode::NOT_FOUND);
    }

    db.execute(
        "INSERT INTO webhook_requests (id, webhook_id, method, headers, body, received_at) VALUES (?, ?, ?, ?, ?, ?)",
        libsql::params![
            id.clone(),
            webhook_id.clone(),
            "POST",
            headers_json.clone(),
            body.clone(),
            received_at.clone()
        ],
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(WebhookRequest {
        id,
        webhook_id,
        method: "POST".to_string(),
        headers: headers_json,
        body,
        received_at,
    }))
}

async fn get_webhook_requests(
    State(state): State<AppState>,
    Path(webhook_id): Path<String>,
) -> Result<Json<Vec<WebhookRequest>>, StatusCode> {
    let db = state.db.lock().await;
    let mut rows = db
        .query(
            "SELECT id, webhook_id, method, headers, body, received_at FROM webhook_requests WHERE webhook_id = ? ORDER BY received_at DESC",
            libsql::params![webhook_id],
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut requests = Vec::new();
    while let Some(row) = rows
        .next()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        requests.push(WebhookRequest {
            id: row.get(0).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
            webhook_id: row.get(1).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
            method: row.get(2).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
            headers: row.get(3).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
            body: row.get(4).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
            received_at: row.get(5).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        });
    }

    Ok(Json(requests))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = std::env::var("TURSO_DATABASE_PATH").expect("TURSO_DATABASE_PATH must be set");

    let db = Builder::new_local(db_path).build().await?.connect()?;

    init_db(&db).await?;

    let state = AppState {
        db: Arc::new(Mutex::new(db)),
    };

    let app = Router::new()
        .route("/api/webhooks", post(create_webhook))
        .route("/api/webhooks", get(list_webhooks))
        .route(
            "/api/webhooks/:webhook_id/requests",
            get(get_webhook_requests),
        )
        .route("/api/webhooks/:webhook_id", post(receive_webhook))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await?;

    Ok(())
}

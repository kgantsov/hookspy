use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Json,
};
use uuid::Uuid;

use crate::app::AppState;
use crate::model::webhook::Webhook;
use crate::schema::webhook::{CreateWebhookRequest, WebhookRequest};

fn construct_url(domain: &str, id: &str) -> String {
    format!("{}/api/webhooks/{}", domain, id)
}

pub async fn create_webhook(
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

    let url = construct_url(&state.domain, &id);

    Ok(Json(Webhook {
        id,
        url,
        name: payload.name,
        created_at,
    }))
}

pub async fn delete_webhook(
    State(state): State<AppState>,
    Path(webhook_id): Path<String>,
) -> Result<(), StatusCode> {
    let db = state.db.lock().await;

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
        "DELETE FROM webhooks WHERE id = ?",
        libsql::params![webhook_id],
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

pub async fn list_webhooks(
    State(state): State<AppState>,
) -> Result<Json<Vec<Webhook>>, StatusCode> {
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
        let id: String = row.get(0).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let url = construct_url(&state.domain, &id);
        let name: String = row.get(1).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let created_at: String = row.get(2).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        webhooks.push(Webhook {
            id,
            url,
            name,
            created_at,
        });
    }

    Ok(Json(webhooks))
}

pub async fn receive_webhook(
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

    let mut notification = state.notification.lock().await;

    let result = WebhookRequest {
        id,
        webhook_id: webhook_id.clone(),
        method: "POST".to_string(),
        headers: headers_json,
        body,
        received_at,
    };

    let result_json =
        serde_json::to_string(&result).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    notification.notify(webhook_id.clone(), result_json).await;

    Ok(Json(result))
}

pub async fn get_webhook_requests(
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

use axum::{
    extract::{ConnectInfo, Path, Query, State},
    http::HeaderMap,
    response::Json,
};
use std::net::SocketAddr;
use std::time::Instant;

use tracing::error;

use crate::dao::webhook::WebhookDao;
#[allow(unused_imports)]
use crate::handlers::error::{ApiError, ErrorBody};
use crate::model::webhook::Webhook;
use crate::schema::webhook::{CreateWebhookRequest, WebhookRequest};
use crate::{app::AppState, auth::jwt::AuthUser};

/// Create a new webhook endpoint
#[utoipa::path(
    post,
    path = "/api/webhooks",
    request_body = CreateWebhookRequest,
    responses(
        (status = 200, description = "Webhook created successfully", body = Webhook),
        (status = 500, description = "Internal server error", body = ErrorBody),
    ),
    security(("cookie_auth" = [])),
    tag = "webhooks"
)]
pub async fn create_webhook(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Json(payload): Json<CreateWebhookRequest>,
) -> Result<Json<Webhook>, ApiError> {
    let db_guard = state.db.lock().await;

    let webhook_dao = WebhookDao {
        domain: state.domain.clone(),
    };

    let webhook = webhook_dao
        .create_webhook(db_guard.clone(), &user.sub, &payload.name)
        .await
        .map_err(|err| {
            error!("Failed to insert webhook: {}", err);
            ApiError::InternalServerError("failed to create webhook".to_string())
        })?;

    Ok(Json(webhook))
}

/// Get a webhook by ID
#[utoipa::path(
    get,
    path = "/api/webhooks/{webhook_id}",
    params(
        ("webhook_id" = String, Path, description = "Unique webhook identifier"),
    ),
    responses(
        (status = 200, description = "Webhook found", body = Webhook),
        (status = 404, description = "Webhook not found", body = ErrorBody),
        (status = 500, description = "Internal server error", body = ErrorBody),
    ),
    security(("cookie_auth" = [])),
    tag = "webhooks"
)]
pub async fn get_webhook(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(webhook_id): Path<String>,
) -> Result<Json<Webhook>, ApiError> {
    let db = state.db.lock().await;

    let webhook_dao = WebhookDao {
        domain: state.domain.clone(),
    };

    let webhook = webhook_dao
        .get_webhook(db.clone(), user.sub.as_str(), webhook_id.as_str())
        .await
        .map_err(|err| {
            error!("Failed to query webhook: {} {}", webhook_id, err);
            ApiError::InternalServerError("failed to get a webhook".to_string())
        })?;

    Ok(Json(webhook))
}

/// List all webhooks for the authenticated user
#[utoipa::path(
    get,
    path = "/api/webhooks",
    responses(
        (status = 200, description = "List of webhooks", body = Vec<Webhook>),
        (status = 500, description = "Internal server error", body = ErrorBody),
    ),
    security(("cookie_auth" = [])),
    tag = "webhooks"
)]
pub async fn list_webhooks(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
) -> Result<Json<Vec<Webhook>>, ApiError> {
    let db = state.db.lock().await;

    let webhook_dao = WebhookDao {
        domain: state.domain.clone(),
    };

    let webhooks = webhook_dao
        .get_webhooks(db.clone(), user.sub.as_str())
        .await
        .map_err(|err| {
            error!("Failed to get webhooks: {}", err);
            ApiError::InternalServerError("failed to fetch webhooks".to_string())
        })?;

    Ok(Json(webhooks))
}

/// Delete a webhook by ID
#[utoipa::path(
    delete,
    path = "/api/webhooks/{webhook_id}",
    params(
        ("webhook_id" = String, Path, description = "Unique webhook identifier"),
    ),
    responses(
        (status = 200, description = "Webhook deleted successfully"),
        (status = 404, description = "Webhook not found", body = ErrorBody),
        (status = 500, description = "Internal server error", body = ErrorBody),
    ),
    security(("cookie_auth" = [])),
    tag = "webhooks"
)]
pub async fn delete_webhook(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(webhook_id): Path<String>,
) -> Result<(), ApiError> {
    let db = state.db.lock().await;

    let webhook_dao = WebhookDao {
        domain: state.domain.clone(),
    };

    let _webhook = webhook_dao
        .get_webhook(db.clone(), user.sub.as_str(), webhook_id.as_str())
        .await
        .map_err(|err| {
            error!("Failed to get webhook: {} {}", webhook_id, err);
            ApiError::NotFound("webhook not found".to_string())
        })?;

    webhook_dao
        .delete_webhook(db.clone(), user.sub.as_str(), webhook_id.as_str())
        .await
        .map_err(|err| {
            error!("Failed to delete webhook: {} {}", webhook_id, err);
            ApiError::InternalServerError("failed to delete webhook".to_string())
        })?;

    Ok(())
}

/// Receive an incoming webhook request
///
/// This is the public endpoint that external services post their webhook payloads to.
/// No authentication is required.
#[utoipa::path(
    post,
    path = "/api/webhooks/{webhook_id}",
    params(
        ("webhook_id" = String, Path, description = "Unique webhook identifier"),
    ),
    request_body(
        content = String,
        description = "Raw webhook payload (any content type)",
        content_type = "text/plain"
    ),
    responses(
        (status = 200, description = "Webhook request recorded", body = WebhookRequest),
        (status = 500, description = "Internal server error", body = ErrorBody),
    ),
    tag = "webhooks"
)]
pub async fn receive_webhook(
    State(state): State<AppState>,
    Path(webhook_id): Path<String>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    body: String,
) -> Result<Json<WebhookRequest>, ApiError> {
    let start = Instant::now();

    // Prefer X-Forwarded-For (set by proxies) over the direct socket address
    let caller_ip = headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.split(',').next().unwrap_or(s).trim().to_string())
        .unwrap_or_else(|| addr.ip().to_string());

    // Convert HeaderMap to a serializable HashMap
    let headers_map: std::collections::HashMap<String, String> = headers
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect();
    let headers_json = serde_json::to_string(&headers_map).unwrap_or_else(|_| "{}".to_string());

    let db = state.db.lock().await;

    let webhook_dao = WebhookDao {
        domain: state.domain.clone(),
    };

    let duration_us = start.elapsed().as_micros() as u64;

    let webhook_request = webhook_dao
        .create_webhook_request(
            db.clone(),
            webhook_id.clone(),
            headers_json.clone(),
            body.clone(),
            Some(caller_ip),
            Some(duration_us),
        )
        .await
        .map_err(|err| {
            error!("Failed to insert webhook request {}", err);
            ApiError::InternalServerError("failed to save a webhook request".to_string())
        })?;

    let user_id = webhook_dao
        .get_webhook_user_id(db.clone(), webhook_id.as_str())
        .await
        .ok();

    let mut notification = state.notification.lock().await;

    let result_json = serde_json::to_string(&webhook_request).map_err(|err| {
        error!("Failed to serialize webhook request {}", err);
        ApiError::InternalServerError("failed to save a webhook request".to_string())
    })?;

    notification.notify(webhook_id.clone(), result_json).await;

    if let Some(uid) = user_id {
        notification.notify_user(&uid, &webhook_id).await;
    }

    Ok(Json(webhook_request))
}

#[derive(serde::Deserialize)]
pub struct PaginationParams {
    pub size: Option<u64>,
    pub page: Option<u64>,
}

/// List all requests received by a webhook
#[utoipa::path(
    get,
    path = "/api/webhooks/{webhook_id}/requests",
    params(
        ("webhook_id" = String, Path, description = "Unique webhook identifier"),
        ("size" = Option<u64>, Query, description = "Number of requests per page (default 100, max 1000)"),
        ("page" = Option<u64>, Query, description = "Page number for pagination (default 1)"),
    ),
    responses(
        (status = 200, description = "List of recorded webhook requests", body = Vec<WebhookRequest>),
        (status = 404, description = "Webhook not found", body = ErrorBody),
        (status = 500, description = "Internal server error", body = ErrorBody),
    ),
    security(("cookie_auth" = [])),
    tag = "webhooks"
)]
pub async fn get_webhook_requests(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(webhook_id): Path<String>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<Vec<WebhookRequest>>, ApiError> {
    let page_size = params.size.unwrap_or(100).min(1000);
    let page_number = params.page.unwrap_or(1).max(1);
    let offset = (page_number - 1) * page_size;

    let db = state.db.lock().await;

    let webhook_dao = WebhookDao {
        domain: state.domain.clone(),
    };

    let _webhook = webhook_dao
        .get_webhook(db.clone(), user.sub.as_str(), webhook_id.as_str())
        .await
        .map_err(|err| {
            error!("Failed to query webhook: {} {}", webhook_id, err);
            ApiError::InternalServerError("failed to get a webhook".to_string())
        })?;

    let requests = webhook_dao
        .get_webhook_requests(db.clone(), webhook_id.as_str(), offset, page_size)
        .await
        .map_err(|err| {
            error!("Failed to fetch webhook requests {}", err);
            ApiError::InternalServerError("failed to fetch webhook requests".to_string())
        })?;

    if let Err(err) = webhook_dao
        .mark_as_seen(db.clone(), user.sub.as_str(), webhook_id.as_str())
        .await
    {
        error!("Failed to mark webhook as seen: {} {}", webhook_id, err);
    }

    Ok(Json(requests))
}

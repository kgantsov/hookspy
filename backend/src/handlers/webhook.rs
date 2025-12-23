use axum::{
    extract::{Path, State},
    http::HeaderMap,
    response::Json,
};

use tracing::error;

use crate::dao::webhook::WebhookDao;
use crate::handlers::error::ApiError;
use crate::model::webhook::Webhook;
use crate::schema::webhook::{CreateWebhookRequest, WebhookRequest};
use crate::{app::AppState, auth::jwt::AuthUser};

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

pub async fn receive_webhook(
    State(state): State<AppState>,
    Path(webhook_id): Path<String>,
    headers: HeaderMap,
    body: String,
) -> Result<Json<WebhookRequest>, ApiError> {
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

    let webhook_request = webhook_dao
        .create_webhook_request(
            db.clone(),
            webhook_id.clone(),
            headers_json.clone(),
            body.clone(),
        )
        .await
        .map_err(|err| {
            error!("Failed to insert webhook request {}", err);
            ApiError::InternalServerError("failed to save a webhook request".to_string())
        })?;

    let mut notification = state.notification.lock().await;

    let result_json = serde_json::to_string(&webhook_request).map_err(|err| {
        error!("Failed to serialize webhook request {}", err);
        ApiError::InternalServerError("failed to save a webhook request".to_string())
    })?;

    notification.notify(webhook_id.clone(), result_json).await;

    Ok(Json(webhook_request))
}

pub async fn get_webhook_requests(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(webhook_id): Path<String>,
) -> Result<Json<Vec<WebhookRequest>>, ApiError> {
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
        .get_webhook_requests(db.clone(), webhook_id.as_str())
        .await
        .map_err(|err| {
            error!("Failed to fetch webhook requests {}", err);
            ApiError::InternalServerError("failed to fetch webhook requests".to_string())
        })?;

    Ok(Json(requests))
}

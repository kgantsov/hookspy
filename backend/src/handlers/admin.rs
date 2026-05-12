use axum::{extract::State, response::Json};

use tracing::error;

#[allow(unused_imports)]
use crate::handlers::error::{ApiError, ErrorBody};
use crate::{app::AppState, auth::jwt::AdminUser};
use crate::{dao::webhook::WebhookDao, model::stats::Stats};

/// List all webhooks for the authenticated user
#[utoipa::path(
    get,
    path = "/api/admin/stats",
    responses(
        (status = 200, description = "Admin statistics", body = Stats),
        (status = 500, description = "Internal server error", body = ErrorBody),
    ),
    security(("cookie_auth" = [])),
    tag = "admin"
)]
pub async fn get_stats(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
) -> Result<Json<Stats>, ApiError> {
    let db = state.db.lock().await;

    let webhook_dao = WebhookDao {
        domain: state.domain.clone(),
    };

    let stats = webhook_dao.get_stats(db.clone()).await.map_err(|err| {
        error!("Failed to get webhooks: {}", err);
        ApiError::InternalServerError("failed to fetch webhooks".to_string())
    })?;

    Ok(Json(stats))
}

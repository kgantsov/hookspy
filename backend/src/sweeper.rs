use std::time::Duration;

use crate::{app::AppState, dao::webhook::WebhookDao};

pub async fn run_sweeper(state: AppState, interval: Duration) {
    let mut ticker = tokio::time::interval(interval);

    ticker.tick().await; // tick immediately to run the first sweep without waiting

    loop {
        ticker.tick().await;
        sweep_once(state.clone()).await;
    }
}

async fn sweep_once(state: AppState) {
    tracing::info!("Sweep old requests...");

    let webhook_dao = WebhookDao {
        domain: state.domain.clone(),
    };

    let db = state.db.lock().await;

    let result = webhook_dao
        .delete_old_webhook_requests(
            db.clone(),
            chrono::Utc::now()
                - chrono::Duration::days(
                    state.config.webhook_retention_days.try_into().unwrap_or(90),
                ),
        )
        .await;

    match result {
        Ok(rows_deleted) => tracing::info!("Successfully swept {:?} requests", rows_deleted),
        Err(e) => tracing::error!("Error sweeping old requests: {:?}", e),
    }

    tracing::info!("Finished sweeping old requests...");
}

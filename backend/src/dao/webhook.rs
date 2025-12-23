use anyhow::Ok;
use uuid::Uuid;

use crate::{model::webhook::Webhook, schema::webhook::WebhookRequest};

pub struct WebhookDao {
    pub domain: String,
}

impl WebhookDao {
    pub fn construct_url(&self, domain: &str, id: &str) -> String {
        format!("{}/api/webhooks/{}", domain, id)
    }

    pub async fn create_webhook(
        &self,
        db: libsql::Connection,
        user_id: &str,
        name: &str,
    ) -> anyhow::Result<Webhook> {
        let id = Uuid::new_v4().to_string();
        let created_at = chrono::Utc::now().to_rfc3339();

        db.execute(
            "INSERT INTO webhooks (id, user_id, name, created_at) VALUES (?, ?, ?, ?)",
            libsql::params![id.clone(), user_id, name, created_at.clone()],
        )
        .await?;

        let url = self.construct_url(&self.domain, &id);

        Ok(Webhook {
            id,
            url,
            name: name.to_string(),
            created_at,
        })
    }
    pub async fn get_webhook(
        &self,
        db: libsql::Connection,
        user_id: &str,
        id: &str,
    ) -> anyhow::Result<Webhook> {
        let mut rows = db
            .query(
                "SELECT id, name, created_at FROM webhooks WHERE user_id = ? AND id = ?",
                libsql::params![user_id, id],
            )
            .await?;

        let row_opt = rows.next().await?;

        let row = match row_opt {
            Some(row) => row,
            None => {
                return Err(anyhow::anyhow!("webhook not found"));
            }
        };

        let id: String = row.get(0)?;
        let name: String = row.get(1)?;
        let created_at: String = row.get(2)?;
        let url = self.construct_url(&self.domain, &id);

        let webhook = Webhook {
            id,
            url,
            name,
            created_at,
        };

        Ok(webhook)
    }

    pub async fn get_webhooks(
        &self,
        db: libsql::Connection,
        user_id: &str,
    ) -> anyhow::Result<Vec<Webhook>> {
        let mut rows = db
            .query(
                "SELECT id, name, created_at FROM webhooks WHERE user_id = ?",
                libsql::params![user_id],
            )
            .await?;

        let mut webhooks = Vec::new();

        while let Some(row) = rows.next().await? {
            let id: String = row.get(0)?;
            let name: String = row.get(1)?;
            let created_at: String = row.get(2)?;
            let url = self.construct_url(&self.domain, &id);

            let webhook = Webhook {
                id,
                url,
                name,
                created_at,
            };

            webhooks.push(webhook);
        }

        Ok(webhooks)
    }

    pub async fn delete_webhook(
        &self,
        db: libsql::Connection,
        user_id: &str,
        id: &str,
    ) -> anyhow::Result<()> {
        db.execute(
            "DELETE FROM webhooks WHERE user_id = ? AND id = ?",
            libsql::params![user_id, id],
        )
        .await?;

        Ok(())
    }

    pub async fn create_webhook_request(
        &self,
        db: libsql::Connection,
        webhook_id: String,
        headers_json: String,
        body: String,
    ) -> anyhow::Result<WebhookRequest> {
        let id = uuid::Uuid::new_v4().to_string();
        let received_at = chrono::Utc::now().to_rfc3339();

        // Verify webhook exists
        let mut rows = db
            .query(
                "SELECT id FROM webhooks WHERE id = ?",
                libsql::params![webhook_id.clone()],
            )
            .await?;

        if rows.next().await?.is_none() {
            return Err(anyhow::anyhow!("webhook not found".to_string(),));
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
        .await?;

        Ok(WebhookRequest {
            id,
            webhook_id: webhook_id.clone(),
            method: "POST".to_string(),
            headers: headers_json,
            body,
            received_at,
        })
    }

    pub async fn get_webhook_requests(
        &self,
        db: libsql::Connection,
        webhook_id: &str,
    ) -> anyhow::Result<Vec<WebhookRequest>> {
        let mut rows = db
            .query(
                "SELECT id, webhook_id, method, headers, body, received_at FROM webhook_requests WHERE webhook_id = ? ORDER BY received_at DESC",
                libsql::params![webhook_id],
            )
            .await?;

        let mut requests = Vec::new();
        while let Some(row) = rows.next().await? {
            let id = row.get(0)?;
            let webhook_id = row.get(1)?;
            let method = row.get(2)?;
            let headers = row.get(3)?;
            let body = row.get(4)?;
            let received_at = row.get(5)?;

            requests.push(WebhookRequest {
                id,
                webhook_id,
                method,
                headers,
                body,
                received_at,
            });
        }

        Ok(requests)
    }
}

use tracing::info;
use uuid::Uuid;

use crate::model::user::User;

pub struct UserDao;

impl UserDao {
    pub async fn create_user(
        &self,
        db: libsql::Connection,
        email: &str,
        first_name: &str,
        last_name: &str,
    ) -> anyhow::Result<User> {
        let id = Uuid::new_v4().to_string();
        let created_at = chrono::Utc::now().to_rfc3339();

        info!(
            "Creating user with email: {} first_name: {} last_name: {} created_at: {}",
            &email, &first_name, &last_name, &created_at
        );

        db.execute(
            "INSERT INTO users (id, email, first_name, last_name, created_at) VALUES (?, ?, ?, ?, ?)",
            libsql::params![id.clone(), email, first_name, last_name, created_at.clone()],
        )
        .await?;

        Ok(User {
            id,
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            email: email.to_string(),
            created_at: created_at.to_string(),
        })
    }

    pub async fn get_user_by_email(
        &self,
        db: libsql::Connection,
        email: &str,
    ) -> anyhow::Result<User> {
        let mut rows = db
            .query(
                "SELECT id, first_name, last_name, email, created_at FROM users WHERE email = ?",
                libsql::params![email],
            )
            .await?;

        let row_opt = rows.next().await?;

        let row = match row_opt {
            Some(row) => row,
            None => {
                return Err(anyhow::anyhow!("user not found"));
            }
        };

        let id: String = row.get(0)?;
        let first_name: String = row.get(1)?;
        let last_name: String = row.get(2)?;
        let email: String = row.get(3)?;
        let created_at: String = row.get(4)?;

        Ok(User {
            id,
            first_name,
            last_name,
            email,
            created_at,
        })
    }
}

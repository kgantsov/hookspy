use turso::Connection;

pub async fn init_db(conn: &Connection) -> Result<(), turso::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            email TEXT NOT NULL,
            created_at TEXT NOT NULL
        )",
        (),
    )
    .await?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS webhooks (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
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
            caller_ip TEXT,
            FOREIGN KEY (webhook_id) REFERENCES webhooks(id) ON DELETE CASCADE
        )",
        (),
    )
    .await?;

    conn.execute("ALTER TABLE webhook_requests ADD COLUMN caller_ip TEXT", ())
        .await
        .ok(); // Ignore error if column already exists

    conn.execute(
        "ALTER TABLE webhook_requests ADD COLUMN duration_us INTEGER",
        (),
    )
    .await
    .ok(); // Ignore error if column already exists

    conn.execute("ALTER TABLE webhooks ADD COLUMN last_seen_at TEXT", ())
        .await
        .ok(); // Ignore error if column already exists

    Ok(())
}

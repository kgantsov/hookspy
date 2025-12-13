use axum::{
    http::{header, StatusCode, Uri},
    response::IntoResponse,
    routing::{delete, get, post},
    Router,
};
use clap::Parser;
use libsql::Builder;
use rust_embed::RustEmbed;
use std::sync::Arc;
use tokio::sync::Mutex;

use hookspy::app::AppState;
use hookspy::handlers::webhook::{
    create_webhook, delete_webhook, get_webhook_requests, list_webhooks, receive_webhook,
};
use hookspy::model::db::init_db;

#[derive(RustEmbed)]
#[folder = "../frontend/dist"]
struct Assets;

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();

    if path.is_empty() {
        path = "index.html".to_string();
    }

    match Assets::get(path.as_str()) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => {
            if let Some(index) = Assets::get("index.html") {
                return ([(header::CONTENT_TYPE, "text/html")], index.data).into_response();
            }
            (StatusCode::NOT_FOUND, "404 Not Found").into_response()
        }
    }
}

#[derive(Parser, Debug)]
#[command(
    author = "Kostiantyn Hantsov",
    about = "Hookspy - Webhook testing tool"
)]
struct Args {
    #[arg(
        short,
        long,
        value_name = "ADDRESS",
        help = "Address to listen on",
        default_value = "0.0.0.0:3000"
    )]
    address: String,

    #[arg(
        short,
        long,
        value_name = "DOMAIN",
        help = "Domain of the webhook server",
        default_value = "http://0.0.0.0:3000"
    )]
    domain: String,

    #[arg(
        short = 'p',
        long,
        value_name = "DATABASE_PATH",
        help = "Path to the database file",
        default_value = "hookspy.db"
    )]
    database_path: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let db_path = args.database_path;

    let db = Builder::new_local(db_path).build().await?.connect()?;

    init_db(&db).await?;

    let state = AppState {
        db: Arc::new(Mutex::new(db)),
        domain: args.domain,
    };

    let api_routes = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/webhooks", post(create_webhook))
        .route("/webhooks", get(list_webhooks))
        .route("/webhooks/:webhook_id/requests", get(get_webhook_requests))
        .route("/webhooks/:webhook_id", post(receive_webhook))
        .route("/webhooks/:webhook_id", delete(delete_webhook));

    let app = Router::new()
        .nest("/api", api_routes)
        .with_state(state)
        .fallback(static_handler);

    let listener = tokio::net::TcpListener::bind(&args.address).await?;
    println!("Server running on http://{}", args.address);

    axum::serve(listener, app).await?;

    Ok(())
}

mod database;

use axum::{routing::get, Json, Router};
use serde_json::json;

#[derive(thiserror::Error, Debug)]
enum SetupError {
    #[error("failed to setup dotenv")]
    DotEnvError(#[from] dotenv::Error),
    #[error("failed to setup tracing")]
    TracingError(#[from] tracing::subscriber::SetGlobalDefaultError),
    #[error("network error")]
    ServerError(#[from] std::io::Error),
}

#[tokio::main]
async fn main() -> Result<(), SetupError> {
    dotenv::dotenv()?;
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;

    let app = Router::new().route(
        "/",
        get(|| async { Json(json!({ "message": "Hello, World!" })) }),
    );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(thiserror::Error, Debug)]
enum SetupError {
    #[error("failed to setup dotenv")]
    DotEnvError(#[from] dotenv::Error),
    #[error("failed to setup tracing")]
    TracingError(#[from] tracing::subscriber::SetGlobalDefaultError),
}

#[tokio::main]
async fn main() -> Result<(), SetupError> {
    dotenv::dotenv()?;
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;

    println!("Hello, world!");
    Ok(())
}

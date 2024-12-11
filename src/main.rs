#[tokio::main]
async fn main() -> Result<(), ()> {
    dotenv::dotenv().expect("failed to setup dotenv");
    pretty_env_logger::init();

    println!("Hello, world!");
    Ok(())
}

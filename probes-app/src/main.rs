use clap::Clap;
use probes_app::{config::Configuration, run};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let config = Configuration::parse();
    run(config).await;
}
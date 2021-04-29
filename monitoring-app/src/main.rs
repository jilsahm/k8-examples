#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    monitoring_app::run().await;
}
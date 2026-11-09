#[tokio::main]
async fn main() {
    tracing::info!("Starting MULX background worker");

    let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));

    loop {
        interval.tick().await;
        tracing::info!("MULX background worker heartbeat");
    }
}

use std::sync::{Arc};
use crate::error::Error;
use super::Server;
use tracing::info;

pub async fn run(_server: Arc<Server>) -> Result<(), Error> {
    info!("UDP Loop");
    tokio::time::sleep(std::time::Duration::from_secs(100)).await;
    Ok(())
}
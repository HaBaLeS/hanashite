mod configuration;
mod server;
mod tcp;
mod udp;
mod error;
mod bus;

use std::path::Path;
use tokio::runtime::Builder;

use tracing::{error, info};
use crate::configuration::Config;

fn main() {
    let config = configuration::init(Path::new("config.toml"));
    configure_logging(&config);
    info!("Setting up Tokio runtime");
    let runtime = {
        let runtime_cfg = &config.runtime;
        Builder::new_multi_thread()
            .worker_threads(runtime_cfg.threads)
            .thread_name(&runtime_cfg.thread_name)
            .thread_stack_size(runtime_cfg.thread_stack)
            .enable_all()
            .build()
            .unwrap()
    };
    info!("Entering Mainloop");
    runtime.block_on(async move {
        mainloop(config).await;
    });
}

async fn mainloop(config: Box<Config>) {
    let server = server::ServerStruct::init(config);
    if let Err(e) = server::ServerStruct::run(server).await {
        error!("Main loop ended with Error: {}", &e);
    } else {
        info!("Main loop ended gracefully.");
    }
}

fn configure_logging(config: &Box<Config>) {
    tracing_subscriber::fmt()
        .with_max_level(config.logging.level)
        .init();
    info!("Logging Configured");
}

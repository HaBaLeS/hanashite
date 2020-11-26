extern crate tokio;
extern crate bytes;

use tokio::runtime::Builder;

mod configuration;
mod controlserver;
mod clienthandler;
mod udphandler;
mod protos;
mod util;

use std::path::Path;

fn main() {
    configuration::init(Path::new("config.toml"));
    configure_logging();
    let config = &configuration::cfg().runtime;
    let runtime = Builder::new_multi_thread()
        .worker_threads(config.threads)
        .thread_name(&config.thread_name)
        .thread_stack_size(config.thread_stack)
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async {
        let server = controlserver::ControlServer::new();
        server.run().await.unwrap();
    });
}

fn configure_logging() {
    let logging_config = &configuration::cfg().logging;
    tracing_subscriber::fmt()
        .with_max_level(logging_config.level)
        .init();
}

#[cfg(test)]
mod tests {
}


use tracing::error;
fn main() {
    tracing_subscriber::fmt().init();
    error!("Test");
}
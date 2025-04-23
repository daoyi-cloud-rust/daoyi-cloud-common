use crate::constants::APP_NAME;
pub use tracing;
pub use tracing_subscriber;

pub async fn app_init() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();
    tracing::info!("{} app_init...finish...", APP_NAME);
    Ok(())
}

pub mod app_init_config;

pub mod config;
pub mod error;

use crate::constants::APP_NAME;
use crate::core::app_init_config::AppInitConfig;
pub use tracing;
use tracing::info;
pub use tracing_subscriber;
use crate::core::config::init;

pub async fn app_init(config: Option<AppInitConfig>) -> Result<(), Box<dyn std::error::Error>> {
    // tracing_subscriber::fmt().init();
    init(config.unwrap_or_default().config);
    info!("{} app_init...finish...", APP_NAME);
    Ok(())
}

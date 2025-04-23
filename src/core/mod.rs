pub use tracing;
pub use tracing_subscriber;
use crate::constants::APP_NAME;

pub fn app_init() {
    tracing_subscriber::fmt().init();
    tracing::info!("{} app_init...finish...", APP_NAME);
}
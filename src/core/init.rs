use crate::constants::APP_NAME;

pub fn init() {
    tracing_subscriber::fmt().init();
    tracing::info!("{} init...finish...", APP_NAME);
}
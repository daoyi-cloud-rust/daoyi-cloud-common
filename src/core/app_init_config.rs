use crate::core::config::ServerConfig;

#[derive(Default)]
pub struct AppInitConfig {
    pub config: Option<ServerConfig>,
}

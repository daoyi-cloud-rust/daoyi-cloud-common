use std::sync::OnceLock;

pub use figment::Figment;
pub use figment::providers::Data;
pub use figment::providers::{Env, Format, Toml};
use serde::Deserialize;
use tracing::{error, info};

mod log_config;
pub use log_config::LogConfig;
mod db_config;
mod redis_config;

pub use db_config::DbConfig;
pub use redis_config::RedisConfig;

pub static CONFIG: OnceLock<ServerConfig> = OnceLock::new();

pub fn common_init(init_toml: Option<Data<Toml>>) {
    let data = Toml::file(Env::var("APP_CONFIG").as_deref().unwrap_or("config.toml"));
    let data2 = Toml::file(Env::var("APP_CONFIG").as_deref().unwrap_or("config.toml"));
    let raw_config = Figment::new()
        .merge(data)
        .merge(Env::prefixed("APP_").global())
        .merge(init_toml.unwrap_or(data2));
    let mut config = match raw_config.extract::<ServerConfig>() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("It looks like your config is invalid. The following error occurred: {e}");
            std::process::exit(1);
        }
    };
    let _guard = config.log.guard();
    info!("log level: {}", &config.log.filter_level);
    if config.db.url.is_empty() {
        config.db.url = std::env::var("DATABASE_URL").unwrap_or_default();
    }
    if config.db.url.is_empty() {
        error!("DATABASE_URL is not set");
        std::process::exit(1);
    }
    CONFIG.set(config).expect("config should be set");
    info!("Config loaded: {:#?}", CONFIG.get().unwrap());
}
pub fn get() -> &'static ServerConfig {
    CONFIG.get().expect("config should be set")
}

#[derive(Deserialize, Clone, Debug)]
pub struct ProfileActiveConfig {
    #[serde(default = "default_profile_active")]
    pub profile_active: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ServerConfig {
    #[serde(default = "default_listen_addr")]
    pub listen_addr: String,

    pub db: DbConfig,
    pub log: LogConfig,
    pub jwt: JwtConfig,
    pub tls: Option<TlsConfig>,
    pub redis: RedisConfig,
}

#[derive(Deserialize, Clone, Debug)]
pub struct JwtConfig {
    pub secret: String,
    pub expiry: i64,
}
#[derive(Deserialize, Clone, Debug)]
pub struct TlsConfig {
    pub cert: String,
    pub key: String,
}

#[allow(dead_code)]
pub fn default_false() -> bool {
    false
}
#[allow(dead_code)]
pub fn default_true() -> bool {
    true
}

fn default_listen_addr() -> String {
    "127.0.0.1:8008".into()
}

fn default_profile_active() -> String {
    "test".into()
}

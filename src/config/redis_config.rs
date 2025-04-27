use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RedisConfig {
    /// Settings for the primary database redis. This is usually writeable, but will be read-only in
    /// some configurations.
    /// An optional follower database. Always read-only.
    #[serde(default = "default_redis_host")]
    pub host: String,
    #[serde(default = "default_redis_port")]
    pub port: u32,
    #[serde(default = "default_redis_db")]
    pub db: u32,
    pub password: Option<String>,
    #[serde(default = "default_redis_pool_max_size")]
    pub pool_max_size: u32,
}

fn default_redis_host() -> String {
    "127.0.0.1".into()
}
fn default_redis_port() -> u32 {
    6379
}
fn default_redis_db() -> u32 {
    0
}
fn default_redis_pool_max_size() -> u32 {
    10
}

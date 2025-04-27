use crate::config::RedisConfig;
use deadpool_redis::redis::cmd;
use deadpool_redis::{Config, Pool, Runtime};
use std::sync::OnceLock;

static REDIS_ENGINE: OnceLock<Pool> = OnceLock::new();
// 初始化连接池
pub async fn init_redis_pool(config: &RedisConfig) {
    let connection_string = match &config.password {
        Some(password) => format!(
            "redis://{}@{}:{}/{}",
            password, config.host, config.port, config.db
        ),
        None => format!("redis://{}:{}/{}", config.host, config.port, config.db),
    };
    let cfg = Config::from_url(connection_string);
    let pool = cfg
        .create_pool(Some(Runtime::Tokio1))
        .expect("Failed to create Redis Pool");
    REDIS_ENGINE.set(pool).expect("redis pool should be set");
}

fn redis_engine() -> &'static Pool {
    REDIS_ENGINE.get().expect("redis should be initialized")
}

async fn get_redis_client() -> deadpool_redis::Connection {
    redis_engine()
        .get()
        .await
        .expect("Failed to get Redis client")
}

pub struct RedisClient {
    client: deadpool_redis::Connection,
}

impl RedisClient {
    pub async fn build() -> Self {
        let client = get_redis_client().await;
        Self { client }
    }

    pub async fn set(&mut self, key: &str, value: &str) {
        cmd("SET")
            .arg(&[key, value])
            .query_async::<()>(&mut self.client)
            .await
            .expect("Failed to set redis value");
    }

    pub async fn get(&mut self, key: &str) -> String {
        let value: String = cmd("GET")
            .arg(&[key])
            .query_async(&mut self.client)
            .await
            .unwrap();
        value
    }
}

#[cfg(test)]
mod tests {
    use crate::config::ServerConfig;
    use crate::redis::RedisClient;
    use crate::{config, redis};
    use config::{Env, Format, Toml};
    use figment::Figment;

    #[tokio::test]
    async fn test_redis_client() {
        let profile = Env::var("APP_PROFILE").unwrap_or_else(|| "test".to_string());
        let data = Toml::file(
            Env::var("APP_CONFIG")
                .as_deref()
                .unwrap_or(format!("config-{}.toml", profile).as_str()),
        );
        let raw_config = Figment::new().merge(data);
        let config = match raw_config.extract::<ServerConfig>() {
            Ok(s) => s,
            Err(e) => {
                eprintln!(
                    "It looks like your config is invalid. The following error occurred: {e}"
                );
                std::process::exit(1);
            }
        };

        redis::init_redis_pool(&config.redis).await;

        let mut redis_client = RedisClient::build().await;
        redis_client.set("deadpool/test_key", "42").await;
        let value: String = redis_client.get("deadpool/test_key").await;
        assert_eq!(value, "42".to_string());
    }
}

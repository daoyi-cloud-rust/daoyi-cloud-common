pub use anyhow;
pub use askama;
pub use dotenvy;
pub use rust_embed;
pub use salvo;
pub use salvo::catcher::Catcher;
pub use salvo::conn::rustls::{Keycert, RustlsConfig};
pub use salvo::prelude::*;
pub use salvo::server::ServerHandle;
pub use serde::Serialize;
pub use tokio;
pub use tokio::signal;
pub use tracing::info;

pub mod common_hoops;
pub mod common_test_routers_example;
pub mod config;
pub mod db;
pub mod models;
pub mod utils;

pub mod error;

pub use error::AppError;
pub mod redis;
pub use redis::RedisClient;

pub type AppResult<T> = Result<T, AppError>;
pub type JsonResult<T> = Result<Json<T>, AppError>;
pub type EmptyResult = Result<Json<Empty>, AppError>;

pub fn json_ok<T>(data: T) -> JsonResult<T> {
    Ok(Json(data))
}
#[derive(Serialize, ToSchema, Clone, Copy, Debug)]
pub struct Empty {}
pub fn empty_ok() -> JsonResult<Empty> {
    Ok(Json(Empty {}))
}

pub async fn shutdown_signal(handle: ServerHandle) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => info!("ctrl_c signal received"),
        _ = terminate => info!("terminate signal received"),
    }
    handle.stop_graceful(std::time::Duration::from_secs(60));
}

#[cfg(test)]
mod tests {
    use config::{Env, Format, Toml};
    use salvo::prelude::*;
    use salvo::test::{ResponseExt, TestClient};

    use crate::config;

    #[tokio::test]
    async fn test_hello_world() {
        let profile = Env::var("APP_PROFILE").unwrap_or_else(|| "test".to_string());
        let data = Toml::file(
            Env::var("APP_CONFIG")
                .as_deref()
                .unwrap_or(format!("config-{}.toml", profile).as_str()),
        );
        config::common_init(Some(data));

        let service = Service::new(crate::common_test_routers_example::root());

        let content = TestClient::get(format!(
            "http://{}",
            config::get().listen_addr.replace("0.0.0.0", "127.0.0.1")
        ))
        .send(&service)
        .await
        .take_string()
        .await
        .unwrap();
        assert_eq!(content, "Hello World from salvo");
    }
}

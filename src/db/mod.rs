use std::sync::OnceLock;

use rbatis::RBatis;

use crate::core::config::DbConfig;

pub static RBATIS_ENGINE: OnceLock<RBatis> = OnceLock::new();

pub async fn init(config: &DbConfig) {
    let rb = RBatis::new();
    rb.init(rbdc_mysql::driver::MysqlDriver {}, &config.url)
        .unwrap();
    RBATIS_ENGINE.set(rb).expect("rbatis should be set");
}

pub fn engine() -> &'static RBatis {
    RBATIS_ENGINE.get().expect("rbatis should be initialized")
}
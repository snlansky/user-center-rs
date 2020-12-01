use actix_web::{App, HttpServer};
use log::info;
use std::sync;

#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate lazy_static;

mod dao;
mod error;
mod handler;
mod model;
mod service;
mod settings;

fn init_logger() {
    use chrono::Local;
    use std::io::Write;

    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    // 设置日志打印格式
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                buf.default_styled_level(record.level()),
                record.module_path().unwrap_or("<unnamed>"),
                &record.args()
            )
        })
        .init();
    info!("env_logger initialized.");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();

    settings::init("config-dev.yaml");
    let config = settings::GLOBAL_CONFIG.read().unwrap();

    dao::init(&config.dao.mongo.uri).await;

    let user_service = service::UserService::new();
    let ctrl = handler::Controller { user_service };
    let ctrl = sync::Arc::new(ctrl);
    HttpServer::new(move || App::new().data(ctrl.clone()).configure(handler::app_config))
        .bind(&config.server.addr)?
        .run()
        .await
}

use actix_web::{App, HttpServer};
use common::config::env_var;
use dotenvy::dotenv;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;

use crate::common::nacos::init_nacos;

pub mod common;
pub mod controller;
pub mod pojo;
pub mod repository;
pub mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect(".env file not found");
    let file_appender =
        tracing_appender::rolling::hourly("./log", format!("{}.log", env!("CARGO_PKG_NAME")));
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let _ = tracing::subscriber::set_global_default(
        tracing_subscriber::fmt::Subscriber::builder()
            .with_max_level(tracing::Level::INFO)
            .finish()
            .with(tracing_subscriber::fmt::Layer::default().with_writer(non_blocking)),
    );
    init_nacos().await;
    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(controller::user_controller::user_rsp)
            .service(controller::user_controller::user_profile)
            .service(controller::sms_config_controller::get_sms_config)
    })
    .bind((
        env_var::<String>("SERVER_HOST"),
        env_var::<u16>("SERVER_PORT"),
    ))?
    .run()
    .await
}

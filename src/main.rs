use actix_web::{App, HttpServer};
use common::config::env_var;
use dotenvy::dotenv;
use tracing_actix_web::TracingLogger;

pub mod common;
pub mod controller;
pub mod pojo;
pub mod repository;
pub mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect(".env file not found");

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(controller::usercontroller::user_profile)
            .service(controller::sms_config_controller::get_sms_config)
    })
    .bind((
        env_var::<String>("SERVER_HOST"),
        env_var::<u16>("SERVER_PORT"),
    ))?
    .run()
    .await
}

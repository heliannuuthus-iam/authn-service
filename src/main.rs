use actix_web::{App, HttpServer};
use common::config::env_var;
use controller::{
    challenge_config_controller, client_controller, idp_controller, password_controller,
    server_controller, sms_config_controller, user_association_controller, user_controller, ApiDoc,
};
use dotenvy::dotenv;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};

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
            .with_max_level(tracing::Level::DEBUG)
            .finish()
            .with(tracing_subscriber::fmt::Layer::default().with_writer(non_blocking)),
    );
    init_nacos().await;
    let api_doc = ApiDoc::openapi();
    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(Redoc::with_url("/openapi", api_doc.clone()))
            .service(password_controller::presist_srp)
            .service(password_controller::user_rsp)
            .service(user_controller::user_profile)
            .service(user_association_controller::user_associations)
            .service(user_association_controller::user_idp_associations)
            .service(user_association_controller::create_user_and_init_idp_asso)
            .service(sms_config_controller::get_sms_config)
            .service(sms_config_controller::set_sms_config)
            .service(challenge_config_controller::set_challenge_config)
            .service(client_controller::create_client)
            .service(client_controller::list_clients)
            .service(client_controller::client_details)
            .service(client_controller::set_client_config)
            .service(client_controller::remove_client)
            .service(server_controller::server_detail)
            .service(server_controller::create_server)
            .service(server_controller::patch_server)
            .service(server_controller::remove_server)
            .service(idp_controller::client_specify_idp_config)
            .service(idp_controller::client_all_idp_config)
            .service(idp_controller::set_client_idp_config)
    })
    .bind((
        env_var::<String>("SERVER_HOST"),
        env_var::<u16>("SERVER_PORT"),
    ))?
    .run()
    .await
}

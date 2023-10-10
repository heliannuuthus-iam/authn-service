use actix_web::error::ErrorNotFound;
use tokio::try_join;

use crate::{
    common::errors::{Result, ServiceError},
    pojo::po::server::{Server, ServerSetting},
    repository::server_repository,
};

pub async fn get_server(server_id: &str) -> Result<Server> {
    match server_repository::select_server(server_id).await? {
        Some(server) => Ok(server),
        None => Err(ServiceError::Reponse(ErrorNotFound(
            "server is nonexistent",
        ))),
    }
}

pub async fn get_server_setting(server_id: &str) -> Result<ServerSetting> {
    let server = get_server(server_id).await?;
    match server_repository::select_server_setting(server_id).await? {
        Some(mut setting) => {
            setting.server = server;
            Ok(setting)
        }
        None => Err(ServiceError::Reponse(ErrorNotFound(
            "server is nonexistent",
        ))),
    }
}

pub async fn set_server(setting: &ServerSetting) -> Result<()> {
    try_join!(
        server_repository::insert_or_update_server(&setting.server),
        server_repository::insert_or_update_server_setting(setting)
    )?;
    Ok(())
}

pub async fn delete_server(server_id: &str) -> Result<()> {
    try_join!(
        server_repository::delete_server(server_id),
        server_repository::delete_server_setting(server_id)
    )?;
    Ok(())
}

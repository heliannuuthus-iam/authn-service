use actix_web::error::ErrorNotFound;
use tokio::try_join;

use crate::{
    common::errors::{Result, ServiceError},
    pojo::po::client::{Client, ClientSetting},
    repository::client_repository,
};

pub async fn get_client(client_id: &str) -> Result<Client> {
    match client_repository::select_client(client_id).await? {
        Some(client) => Ok(client),
        None => Err(ServiceError::Reponse(ErrorNotFound(
            "client is nonexsitent",
        ))),
    }
}

pub async fn get_client_setting(client_id: &str) -> Result<ClientSetting> {
    let client = get_client(client_id).await?;

    match client_repository::select_client_setting(client_id).await? {
        Some(mut setting) => {
            setting.client = client;
            Ok(setting)
        }
        None => Err(ServiceError::Reponse(ErrorNotFound(
            "client setting is nonexsitent",
        ))),
    }
}

pub async fn set_client_setting(setting: &ClientSetting) -> Result<()> {
    try_join!(
        client_repository::insert_or_update_client(&setting.client),
        client_repository::insert_or_update_client_setting(setting)
    )?;
    Ok(())
}

pub async fn remove_client(client_id: &str) -> Result<()> {
    try_join!(
        client_repository::delete_client(client_id),
        client_repository::delete_client_setting(client_id)
    )?;
    Ok(())
}

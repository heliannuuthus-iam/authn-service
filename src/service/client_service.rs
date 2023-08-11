use crate::{
    common::errors::{Result, ServiceError},
    pojo::po::client::ClientConfig,
    repository::client_repository,
};

pub async fn get_client_config(client_id: &str) -> Result<ClientConfig> {
    client_repository::select_client_config(client_id)
        .await
        .and_then(|cli_op| match cli_op {
            Some(client) => Ok(client),
            None => Err(ServiceError::NotFount(format!("client is nonexsitent"))),
        })
}

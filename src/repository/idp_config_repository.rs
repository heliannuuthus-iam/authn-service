use anyhow::Context;

use crate::{
    common::{datasource::CONN, enums::IdpType, errors::Result},
    pojo::po::client::ClientIdpConfig,
};

pub async fn select_specify_idp_config(
    client_id: &str,
    idp_type: IdpType,
) -> Result<Option<ClientIdpConfig>> {
    Ok(sqlx::query_as!(
        ClientIdpConfig,
        "SELECT idp_type, idp_client_id, idp_client_secret FROM t_client_idp_config WHERE \
         client_id = ? and idp_type = ?",
        client_id,
        idp_type
    )
    .fetch_optional(&*CONN)
    .await
    .with_context(|| {
        tracing::error!("fetch specify idp config failed. client_id: {}", client_id);
        "fetch specify idp config failed"
    })?)
}

pub async fn select_client_idp_config(client_id: &str) -> Result<Vec<ClientIdpConfig>> {
    Ok(sqlx::query_as!(
        ClientIdpConfig,
        "SELECT idp_type, idp_client_id, idp_client_secret FROM t_client_idp_config WHERE \
         client_id = ?",
        client_id
    )
    .fetch_all(&*CONN)
    .await
    .with_context(|| {
        tracing::error!("fetch specify idp config failed, client_id: {}", client_id);
        "fetch specify idp config failed"
    })?)
}

pub async fn save_or_update_client_idp_config(
    client_id: &str,
    idp_config: &ClientIdpConfig,
) -> Result<()> {
    sqlx::query!(
        "INSERT INTO t_client_idp_config(client_id, idp_type, idp_client_id, idp_client_secret) \
         VALUES(?, ?, ?, ?) ON DUPLICATE KEY UPDATE idp_client_secret = ?",
        client_id,
        idp_config.idp_type,
        idp_config.idp_client_id,
        idp_config.idp_client_secret,
        idp_config.idp_client_secret,
    )
    .execute(&*CONN)
    .await
    .with_context(|| {
        tracing::error!("fetch specify idp config failed, client_id: {}", client_id);
        "fetch specify idp config failed"
    })?;
    Ok(())
}

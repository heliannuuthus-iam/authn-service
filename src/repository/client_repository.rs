use anyhow::Context;

use crate::{
    common::{datasource::CONN, errors::Result},
    pojo::po::client::ClientConfig,
};

pub async fn select_client_config(client_id: &str) -> Result<Option<ClientConfig>> {
    Ok(sqlx::query_as!(
        ClientConfig,
        "SELECT * FROM t_client_config WHERE client_id = ?",
        client_id
    )
    .fetch_optional(&*CONN)
    .await
    .with_context(|| {
        let msg = format!("select client config");
        tracing::error!(msg);
        msg
    })?)
}

pub async fn create_client(config: &ClientConfig) -> Result<()> {
    sqlx::query!(
        "INSERT INTO t_client_config(client_id, name, logo, description) VALUES(?, ?, ?, ?)",
        config.client_id,
        config.name,
        config.logo,
        config.description
    )
    .execute(&*CONN)
    .await
    .with_context(|| {
        let msg = format!("create client failed");
        tracing::error!(msg);
        msg
    })?;
    Ok(())
}

pub async fn set_client_config(config: &ClientConfig) -> Result<()> {
    sqlx::query!(
        "INSERT INTO t_client_config(client_id, name, logo, description) VALUES(?, ?, ?, ?) ON \
         DUPLICATE KEY UPDATE name = ?, logo = ?, description = ?, redirect_url = ?",
        config.client_id,
        config.name,
        config.logo,
        config.description,
        config.name,
        config.logo,
        config.description,
        config.redirect_url
    )
    .execute(&*CONN)
    .await
    .with_context(|| {
        let msg = format!("set client config failed");
        tracing::error!(msg);
        msg
    })?;
    Ok(())
}

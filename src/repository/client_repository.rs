use anyhow::Context;
use sqlx::MySql;

use crate::{
    common::{datasource::CONN, errors::Result},
    pojo::po::client::{Client, ClientSetting},
};

pub async fn select_client(client_id: &str) -> Result<Option<Client>> {
    Ok(sqlx::query_as!(
        Client,
        "SELECT client_id, name, logo, description, client_type FROM t_client WHERE client_id = ?",
        client_id
    )
    .fetch_optional(&*CONN)
    .await
    .with_context(|| {
        tracing::error!("select client({}) config", client_id);
        "select client config"
    })?)
}

pub async fn select_client_setting(client_id: &str) -> Result<Option<ClientSetting>> {
    Ok(sqlx::query_as::<MySql, ClientSetting>(
        "SELECT client_id, callbacks, allowed_origins, allowed_logout_urls FROM t_client_setting \
         WHERE client_id = ?",
    )
    .bind(client_id)
    .fetch_optional(&*CONN)
    .await
    .with_context(|| {
        tracing::error!("select client({}) config", client_id);
        "select client config"
    })?)
}

pub async fn insert_or_update_client(client: &Client) -> Result<()> {
    sqlx::query!(
        "INSERT INTO t_client(client_id, name, logo, description) VALUES(?, ?, ?, ?) ON DUPLICATE \
         KEY UPDATE name = ?, logo = ?, description = ?",
        client.client_id,
        client.name,
        client.logo,
        client.description,
        client.name,
        client.logo,
        client.description
    )
    .execute(&*CONN)
    .await
    .with_context(|| {
        tracing::error!("create client({:?}) failed", client);
        "create client failed"
    })?;
    Ok(())
}

pub async fn insert_or_update_client_setting(setting: &ClientSetting) -> Result<()> {
    sqlx::query!(
        "INSERT INTO t_client_setting(client_id, callbacks, allowed_origins, allowed_logout_urls) \
         VALUES(?, ?, ?, ?) ON DUPLICATE KEY UPDATE callbacks = ?, allowed_origins = ?, \
         allowed_logout_urls = ?",
        setting.client_id,
        setting.callbacks,
        setting.allowed_origins,
        setting.allowed_logout_urls,
        setting.callbacks,
        setting.allowed_origins,
        setting.allowed_logout_urls
    )
    .execute(&*CONN)
    .await
    .with_context(|| {
        tracing::error!("set client setting({:?}) failed", setting);
        "set client setting failed"
    })?;
    Ok(())
}

pub async fn delete_client(client_id: &str) -> Result<()> {
    sqlx::query("DELETE FROM t_client WHERE client_id = ?")
        .bind(client_id)
        .execute(&*CONN)
        .await
        .with_context(|| {
            tracing::error!("delete client({}) failed", client_id);
            "delete client failed"
        })?;
    Ok(())
}

pub async fn delete_client_setting(client_id: &str) -> Result<()> {
    sqlx::query("DELETE FROM t_client_setting WHERE client_id = ?")
        .bind(client_id)
        .execute(&*CONN)
        .await
        .with_context(|| {
            tracing::error!("delete client({}) setting failed", client_id);
            "delete client setting failed"
        })?;
    Ok(())
}

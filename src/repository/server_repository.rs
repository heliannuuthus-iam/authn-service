use anyhow::Context;
use sqlx::MySql;

use crate::{
    common::{datasource::CONN, errors::Result},
    pojo::po::server::{Server, ServerSetting},
};

pub async fn select_server(server_id: &str) -> Result<Option<Server>> {
    Ok(sqlx::query_as!(
        Server,
        "SELECT server_id, name, logo, description FROM t_server WHERE server_id = ?",
        server_id
    )
    .fetch_optional(&*CONN)
    .await
    .with_context(|| {
        let msg = format!("select server failed {}", server_id);
        tracing::error!(msg);
        msg
    })?)
}

pub async fn select_server_setting(server_id: &str) -> Result<Option<ServerSetting>> {
    Ok(sqlx::query_as::<MySql, ServerSetting>(
        "SELECT server_id, allow_offline_access, token_lifttime, signing_alg FROM \
         t_server_setting WHERE server_id = ?",
    )
    .bind(server_id)
    .fetch_optional(&*CONN)
    .await
    .with_context(|| {
        let msg = format!("select server setting failed {}", server_id);
        tracing::error!(msg);
        msg
    })?)
}

pub async fn insert_or_update_server(server: &Server) -> Result<()> {
    sqlx::query!(
        "INSERT INTO t_server(server_id, name, logo, description) VALUES(?, ?, ? ,?) ON DUPLICATE \
         KEY UPDATE name = ?, logo = ?, description = ?",
        server.server_id,
        server.name,
        server.logo,
        server.description,
        server.name,
        server.logo,
        server.description,
    )
    .execute(&*CONN)
    .await
    .with_context(|| {
        let msg = format!("set server failed {:?}", server);
        tracing::error!(msg);
        msg
    })?;
    Ok(())
}

pub async fn insert_or_update_server_setting(setting: &ServerSetting) -> Result<()> {
    sqlx::query!(
        "INSERT INTO t_server_setting(server_id, allow_offline_access, token_lifttime, \
         signing_alg) VALUES(?, ?, ? ,?) ON DUPLICATE KEY UPDATE allow_offline_access = ?, \
         token_lifttime = ?, signing_alg = ?",
        setting.server_id,
        setting.allow_offline_access,
        setting.token_lifttime,
        setting.signing_alg,
        setting.allow_offline_access,
        setting.token_lifttime,
        setting.signing_alg
    )
    .execute(&*CONN)
    .await
    .with_context(|| {
        let msg = format!("set server failed {:?}", setting);
        tracing::error!(msg);
        msg
    })?;
    Ok(())
}

pub async fn delete_server(server_id: &str) -> Result<()> {
    sqlx::query("DELETE FROM t_server WHERE server_id = ?")
        .bind(server_id)
        .execute(&*CONN)
        .await
        .with_context(|| {
            let msg = format!("delete server({}) failed", server_id);
            tracing::error!(msg);
            msg
        })?;
    Ok(())
}

pub async fn delete_server_setting(server_id: &str) -> Result<()> {
    sqlx::query("DELETE FROM t_server_setting WHERE server_id = ?")
        .bind(server_id)
        .execute(&*CONN)
        .await
        .with_context(|| {
            let msg = format!("delete server({}) setting failed", server_id);
            tracing::error!(msg);
            msg
        })?;
    Ok(())
}

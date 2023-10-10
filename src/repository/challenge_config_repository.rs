use anyhow::Context;

use crate::{
    common::{datasource::CONN, errors::Result},
    pojo::po::challenge::ChallengeCofig,
};

pub async fn select_challenge_configs(client_id: &str) -> Result<Vec<ChallengeCofig>> {
    Ok(sqlx::query_as!(
        ChallengeCofig,
        "SELECT client_id, name, type as challenge_type FROM t_challenge_config WHERE client_id = \
         ? ",
        client_id,
    )
    .fetch_all(&*CONN)
    .await
    .with_context(|| {
        let msg = format!("select challenge configs failed client_id: {client_id}");
        tracing::error!(msg);
        msg
    })?)
}

pub async fn save_challenge_config(config: &ChallengeCofig) -> Result<()> {
    sqlx::query!(
        "INSERT INTO t_challenge_config(client_id, name, type) VALUES(?, ?, ?)",
        config.client_id,
        config.name,
        config.challenge_type,
    )
    .execute(&*CONN)
    .await
    .with_context(|| {
        let msg = format!("save challenge config failed config: {:?}", config);
        tracing::error!(msg);
        msg
    })?;
    Ok(())
}

pub async fn select_challenge_config(
    client_id: &str,
    name: &str,
    challenge_type: &str,
) -> Result<Option<ChallengeCofig>> {
    Ok(sqlx::query_as!(
        ChallengeCofig,
        "SELECT client_id, name, type as challenge_type FROM t_challenge_config WHERE client_id = \
         ? and name = ? and type = ? ",
        client_id,
        name,
        challenge_type,
    )
    .fetch_optional(&*CONN)
    .await
    .with_context(|| {
        let msg = format!("sele challenge config failed client_id: {:?}", client_id);
        tracing::error!(msg);
        msg
    })?)
}

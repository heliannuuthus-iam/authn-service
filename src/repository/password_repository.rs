use anyhow::Context;

use crate::{
    common::{datasource::CONN, errors::Result},
    pojo::po::srp::SrpPassword,
};

pub async fn save_srp(identifier: String, verifier: String, salt: String) -> Result<()> {
    sqlx::query!(
        "INSERT INTO t_srp_password(identifier, verifier, salt) VALUES(?, ?, ?)",
        identifier,
        verifier,
        salt
    )
    .fetch_one(&*CONN)
    .await
    .context("save srp password failed")?;
    Ok(())
}

pub async fn select_srp(identifier: String) -> Result<Option<SrpPassword>> {
    Ok(sqlx::query_as!(
        SrpPassword,
        "SELECT identifier, verifier, salt FROM t_srp_password where identifier = ?",
        identifier
    )
    .fetch_optional(&*CONN)
    .await
    .context("select srp failed")?)
}

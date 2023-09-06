use anyhow::Context;

use crate::{
    common::{datasource::CONN, errors::Result},
    pojo::po::sms_config::SmsConfig,
};

pub async fn select_sms_config(id: i64) -> Result<Option<SmsConfig>> {
    Ok(sqlx::query_as!(
        SmsConfig,
        "SELECT name, template FROM t_sms_config where id = ?",
        id
    )
    .fetch_optional(&*CONN)
    .await
    .context("select sms config failed")?)
}

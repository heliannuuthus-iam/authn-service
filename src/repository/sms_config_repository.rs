use anyhow::Context;

use crate::{
    common::{datasource::CONN, errors::Result},
    pojo::po::sms::SmsConfig,
};

pub async fn select_sms_config(template_id: &str) -> Result<Option<SmsConfig>> {
    Ok(sqlx::query_as!(
        SmsConfig,
        "SELECT name, template_id, template FROM t_sms_config where template_id = ?",
        template_id
    )
    .fetch_optional(&*CONN)
    .await
    .context("select sms config failed")?)
}

pub async fn save_or_update_config(config: &SmsConfig) -> Result<()> {
    sqlx::query!(
        "INSERT INTO t_sms_config(name, template_id, template) VALUES (?, ?, ?) ON DUPLICATE KEY \
         UPDATE name = ?, template = ?",
        config.name,
        config.template_id,
        config.template,
        config.name,
        config.template
    )
    .execute(&*CONN)
    .await
    .context("set sms config failed")?;

    Ok(())
}

use actix_web::{get, web::Path};

use crate::{
    common::errors::{Resp, Result, ServiceError},
    pojo::po::sms_config::SmsConfig,
    repository::sms_config_repository::select_sms_config,
};

#[get("/smsconfig/{id}")]
pub async fn get_sms_config(id: Path<i64>) -> Result<Resp<SmsConfig>> {
    select_sms_config(*id)
        .await
        .and_then(|sms_config| {
            sms_config.ok_or(ServiceError::NotFount(String::from("sms_config not found")))
        })
        .map(Resp::success)
}

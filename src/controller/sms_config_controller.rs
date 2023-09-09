use actix_web::{
    get, post,
    web::{Json, Path},
    Responder,
};

use crate::{
    common::errors::{Result, ServiceError},
    pojo::po::sms::SmsConfig,
    repository::sms_config_repository::{self},
};

#[utoipa::path(
    operation_id = "查询 sms 配置模版信息",
    params(
        ("template_id" = String, Path,)
    ),
    responses(
        (status = 200, description = "OK", body = SmsConfig),
    )
)]
#[get("/smsconfig/{template_id}")]
pub async fn get_sms_config(template_id: Path<String>) -> Result<impl Responder> {
    sms_config_repository::select_sms_config(&template_id)
        .await
        .and_then(|sms_config| {
            sms_config.ok_or(ServiceError::ReponseError(actix_web::error::ErrorNotFound(
                "sms_config not found",
            )))
        })
        .map(Json)
}

#[utoipa::path(
    operation_id = "设置 sms 配置模版信息",
    request_body  = SmsConfig,
    responses(
        (status = 200, description = "OK", body = SmsConfig),
    )
)]
#[post("/smsconfig")]
pub async fn set_sms_config(Json(form): Json<SmsConfig>) -> Result<impl Responder> {
    Ok(Json(
        sms_config_repository::save_or_update_config(&form).await?,
    ))
}

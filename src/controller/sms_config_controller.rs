use actix_web::{
    get,
    web::{Json, Path},
    Responder,
};

use crate::{
    common::errors::{Result, ServiceError},
    repository::sms_config_repository::select_sms_config,
};

#[utoipa::path(
    operation_id = "新增 sms 模版信息",
    params(
        ("id" = i64, Path,)
    ),
    responses(
        (status = 200, description = "OK", body = SmsConfig),
    )
)]
#[get("/smsconfig/{id}")]
pub async fn get_sms_config(id: Path<i64>) -> Result<impl Responder> {
    select_sms_config(*id)
        .await
        .and_then(|sms_config| {
            sms_config.ok_or(ServiceError::ReponseError(actix_web::error::ErrorNotFound(
                "sms_config not found",
            )))
        })
        .map(Json)
}

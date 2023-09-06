use actix_web::{
    error::ErrorNotFound,
    get, post,
    web::{Form, Json, Path},
    HttpResponse, Responder,
};
use tracing::info;

use crate::{
    common::errors::{Result, ServiceError},
    pojo::form::user::RegistryForm,
    repository::password_repository::{save_srp, select_srp},
    service::user_service::get_user,
};

#[utoipa::path(
    operation_id = "根据用户标识, 查询用户 id",
    params(
        ("openid" = String, Path,)
    ),
    responses(
        (status = 200, description = "OK", body = User),
        (status = 404, description = "用户不存在"),
    )
)]
#[get("/users/{openid}")]
pub async fn user_profile(openid: Path<String>) -> Result<impl Responder> {
    info!("[查询用户信息]: {openid}");
    match user_service::get_user(&openid, true).await? {
        Some(u) => Ok(Json(u)),
        None => Err(ServiceError::ReponseError(ErrorNotFound("user not fount"))),
    }
}

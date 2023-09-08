use actix_web::{
    error::ErrorNotFound,
    get, post,
    web::{Form, Json, Path},
    HttpResponse, Responder,
};
use tracing::info;

use crate::{
    common::errors::{Result, ServiceError},
    pojo::form::user::SrpPasswordForm,
    repository::password_repository,
    service::user_service::{self},
};

#[utoipa::path(
    operation_id = "储存 srp 密码, 应当先创建用户关联关系",
    params(
        SrpPasswordForm
    ),
    responses(
        (status = 200, description = "OK"),
    )
)]
#[post("/registry")]
pub async fn registry(Form(registry_form): Form<SrpPasswordForm>) -> Result<impl Responder> {
    password_repository::save_srp(
        registry_form.identifier,
        registry_form.verifier,
        registry_form.salt,
    )
    .await?;
    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
    operation_id = "获取 srp 信息",
    params(
        ("identifier" = String, Path,)
    ),
    responses(
        (status = 200, description = "OK", body = SrpPassword),
        (status = 404, description = "srp 信息不存在"),
    )
)]
#[get("/users/rsp/{identifier}")]
pub async fn user_rsp(identifier: Path<String>) -> Result<impl Responder> {
    info!("[获取 srp 信息]: {identifier}");
    password_repository::select_srp(identifier.into_inner())
        .await
        .and_then(|srp| {
            srp.ok_or(ServiceError::ReponseError(ErrorNotFound(
                "select srp failed",
            )))
        })
        .map(Json)
}

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

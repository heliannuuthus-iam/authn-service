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
    repository::password_repository,
    service::user_service::{self},
};

#[utoipa::path(
    params(
        RegistryForm
    ),
    responses(
        (status = 200, description = "OK"),
    )
)]
#[post("/registry")]
pub async fn registry(Form(registry_form): Form<RegistryForm>) -> Result<impl Responder> {
    password_repository::save_srp(
        registry_form.identifier,
        registry_form.verifier,
        registry_form.salt,
    )
    .await?;
    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
    params(
        ("identifier" = String, Path,)
    ),
    responses(
        (status = 200, description = "OK", body = SrpPassword),
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
    params(
        ("openid" = String, Path,)
    ),
    responses(
        (status = 200, description = "OK", body = User),
    )
)]
#[get("/users/{openid}")]
pub async fn user_profile(openid: Path<String>) -> Result<impl Responder> {
    info!("[查询用户信息]: {openid}");
    user_service::get_user(&openid, true).await.map(Json)
}

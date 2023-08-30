use actix_web::{
    get, post,
    web::{Form, Path},
    Responder,
};
use serde;
use tracing::info;

use crate::{
    common::errors::{Resp, Result, ServiceError},
    repository::password_repository::{save_srp, select_srp},
    service::user_service::get_user,
};

#[derive(serde::Deserialize, Debug)]
pub struct ProfileQuery(
    #[serde(rename = "openid")] pub Option<String>,
    #[serde(rename = "email")] pub Option<String>,
);

#[derive(serde::Deserialize, Debug)]
pub struct RegistryForm(
    #[serde(rename = "identifier")] pub String,
    #[serde(rename = "verifier")] pub String,
    #[serde(rename = "salt")] pub String,
);

#[post("/registry")]
pub async fn registry(Form(registry_form): Form<RegistryForm>) -> Result<impl Responder> {
    save_srp(registry_form.0, registry_form.1, registry_form.2)
        .await
        .map(Resp::success)
}

#[get("/users/rsp/{identifier}")]
pub async fn user_rsp(identifier: Path<String>) -> Result<impl Responder> {
    info!("[获取 srp 信息]: {identifier}");
    select_srp(identifier.into_inner())
        .await
        .and_then(|srp| srp.ok_or(ServiceError::NotFount(format!("select srp failed"))))
        .map(Resp::success)
}

#[get("/users/{openid}")]
pub async fn user_profile(openid: Path<String>) -> Result<impl Responder> {
    info!("[查询用户信息]: {openid}");
    get_user(&openid).await.map(Resp::success)
}

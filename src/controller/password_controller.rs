use actix_web::{
    error::ErrorNotFound,
    get, post,
    web::{Json, Path},
    HttpResponse, Responder,
};

use crate::{
    common::{
        errors::{Result, ServiceError},
        utils::desensitize_text,
    },
    pojo::form::user::SrpPasswordForm,
    repository::password_repository,
};

#[utoipa::path(
    operation_id = "新增 srp 信息",
    request_body = SrpPasswordForm,
    responses(
        (status = 200, description = "OK"),
    )
)]
#[post("/password/srp")]
pub async fn presist_srp(Json(form): Json<SrpPasswordForm>) -> Result<impl Responder> {
    password_repository::save_srp(&form.identifier, &form.verifier, &form.salt).await?;
    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
    operation_id="根据身份查询 srp 信息",
    params(
        ("identifier" = String, Path,)
    ),
    responses(
        (status = 200, description = "OK", body = SrpPassword),
    )
)]
#[get("/password/srp/{identifier}")]
pub async fn user_rsp(identifier: Path<String>) -> Result<impl Responder> {
    let identifier = identifier.into_inner();
    tracing::info!("{} 获取 srp 信息", desensitize_text(&identifier));
    password_repository::select_srp(&identifier)
        .await
        .and_then(|srp| {
            srp.ok_or(ServiceError::ReponseError(ErrorNotFound(format!(
                "select srp failed"
            ))))
        })
        .map(Json)
}

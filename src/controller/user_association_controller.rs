use actix_web::{
    get, post,
    web::{Json, Path},
    Responder,
};
use tracing::info;

use crate::{
    common::errors::Result,
    pojo::{dto::user::UserProfileDTO, form::user::UserAssoInitialForm},
    service::{user_association_service, user_service},
};

#[utoipa::path(
    operation_id = "根据 open_id 查询用户关联信息",
    params(
        ("openid" = String, Path,)
    ),
    responses(
        (status = 200, description = "OK", body = UserProfileDTO),
    )
)]
#[get("/associations/{openid}")]
pub async fn user_associations(openid: Path<String>) -> Result<impl Responder> {
    info!("【根据 openid 查询关联身份】openid({}) ", openid);

    user_association_service::get_profile_with_associations(&openid)
        .await
        .map(Json)
}

#[utoipa::path(
    operation_id = "根据 idp_openid 查询反查用户关联信息",
    params(
        ("idp_openid" = String, Path,)
    ),
    responses(
        (status = 200, description = "OK", body = UserProfileDTO),
    )
)]
#[get("/associations/{idp_openid}/idp")]
pub async fn user_idp_associations(idp_openid: Path<String>) -> Result<impl Responder> {
    info!(
        "【根据 idp_openid 查询关联身份】idp_openid({}) ",
        idp_openid
    );
    user_association_service::get_profile_by_idp_openid(&idp_openid)
        .await
        .map(Json)
}

#[utoipa::path(
    operation_id = "创建用户关联关系, 用于 oauth 登录 + 注册",
    request_body = UserAssoInitialForm,
    responses(
        (status = 200, description = "OK", body = UserProfileDTO),
    )
)]
#[post("/associations")]
pub async fn create_user_and_init_idp_asso(
    Json(form): Json<UserAssoInitialForm>,
) -> Result<impl Responder> {
    tracing::info!("创建用户关联关系:{:?}", form);
    let (user, mut associations) = if let Some(user) =
        &user_service::get_user(&form.identifier, false).await?
    {
        (
            user.clone(),
            user_association_service::enstablish_idp_association(&user.openid, &form.association)
                .await?,
        )
    } else {
        user_service::create_user(&form.identifier, Some(&form.association)).await?
    };
    let mut profile: UserProfileDTO = user.into();
    profile.associations.append(&mut associations);
    Ok(Json(profile))
}

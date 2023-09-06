use actix_web::{
    get,
    web::{Json, Path},
    Responder,
};
use tracing::info;

use crate::{common::errors::Result, service::user_association_service};

#[utoipa::path(
    params(
        ("openid" = String, Path,)
    ),
    responses(
        (status = 200, description = "OK", body = UserProfileDTO),
    )
)]
#[get("/users/associations/{openid}")]
pub async fn user_associations(openid: Path<String>) -> Result<impl Responder> {
    info!("【根据 openid 查询关联身份】openid({}) ", openid);

    user_association_service::get_profile_with_associations(&openid)
        .await
        .map(Json)
}

#[utoipa::path(
    params(
        ("idp_openid" = String, Path,)
    ),
    responses(
        (status = 200, description = "OK", body = UserProfileDTO),
    )
)]
#[get("/users/associations/{idp_openid}/idp")]
pub async fn user_idp_associations(idp_openid: Path<String>) -> Result<impl Responder> {
    info!(
        "【根据 idp_openid 查询关联身份】idp_openid({}) ",
        idp_openid
    );
    user_association_service::get_profile_by_idp_openid(&idp_openid)
        .await
        .map(Json)
}

use actix_web::{get, web::Path, Responder};
use tracing::info;

use crate::{
    common::errors::{Resp, Result},
    service::user_service::{get_profile_by_idp_openid, get_profile_with_associations},
};

#[get("/users/associations/{openid}")]
pub async fn user_associations(openid: Path<String>) -> Result<impl Responder> {
    info!("【根据 openid 查询关联身份】openid({}) ", openid);
    get_profile_with_associations(&openid)
        .await
        .map(Resp::success)
}

#[get("/users/associations/{idp_openid}/idp")]
pub async fn user_idp_associations(idp_openid: Path<String>) -> Result<impl Responder> {
    info!(
        "【根据 idp_openid 查询关联身份】idp_openid({}) ",
        idp_openid
    );
    get_profile_by_idp_openid(&idp_openid)
        .await
        .map(Resp::success)
}

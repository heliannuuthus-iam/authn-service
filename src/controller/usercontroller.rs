use actix_web::{
    get,
    web::{self},
};
use serde;
use tracing::info;

use crate::{
    common::errors::{Resp, Result},
    pojo::po::user::User,
    service::userservice::get_profile,
};
#[derive(serde::Deserialize, Debug)]
pub struct ProfileQuery {
    pub email: Option<String>,
    pub openid: Option<String>,
}

#[get("/users/profile")]
pub async fn user_profile(web::Query(query): web::Query<ProfileQuery>) -> Result<Resp<User>> {
    info!("查询用户信息: {:?}", query);
    Ok(get_profile(query).await.map(Resp::success)?)
}

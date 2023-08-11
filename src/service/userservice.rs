use crate::{
    common::errors::{Result, ServiceError},
    controller::usercontroller::ProfileQuery,
    pojo::po::user::User,
    repository::user_repository::select_profile_by_union,
};

pub async fn get_profile(query: ProfileQuery) -> Result<User> {
    select_profile_by_union(query.openid, query.email)
        .await
        .and_then(|u| u.ok_or(ServiceError::NotFount(String::from("user not found"))))
}

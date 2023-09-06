use anyhow::Context;
use chrono::Duration;

use crate::{
    common::{
        cache::{cache_get, cache_setex},
        errors::{Result, ServiceError},
    },
    pojo::{
        dto::user::{UserAssociationDTO, UserProfileDTO},
        po::user::User,
    },
    repository::{
        user_association_repository::{
            select_user_associations, select_user_associations_by_idp_openid,
        },
        user_repository::select_profile,
    },
};

const USER_PROFILE_CACHE_KEY: &str = "forum:user:profile:";

pub async fn get_user(openid: &str) -> Result<User> {
    if let Some(u) =
        cache_get::<User>(format!("{}{}", USER_PROFILE_CACHE_KEY, openid).as_str()).await?
    {
        Ok(u)
    } else {
        let user = select_profile(Some(openid.to_string()), None)
            .await
            .and_then(|u| u.ok_or(ServiceError::NotFount(String::from("user not found"))))?;

        cache_setex(
            format!("{}{}", USER_PROFILE_CACHE_KEY, openid).as_str(),
            user.clone(),
            Duration::days(1),
        )
        .await?;

        Ok(user)
    }
}

pub async fn get_profile_by_idp_openid(idp_openid: &str) -> Result<UserProfileDTO> {
    let (openid, associations): (String, Vec<UserAssociationDTO>) =
        select_user_associations_by_idp_openid(idp_openid).await?;
    let mut user = UserProfileDTO::from(get_user(&openid).await?);
    user.associations = associations.clone();
    Ok(user)
}

pub async fn get_profile_with_associations(openid: &str) -> Result<UserProfileDTO> {
    let user = get_user(openid).await?;
    let user_profile = select_user_associations(&user.openid)
        .await
        .map(|associations| {
            let mut user = UserProfileDTO::from(user);
            user.associations = associations.clone();
            user
        })
        .context(format!("get user({openid}) association failed"))?;
    Ok(user_profile)
}

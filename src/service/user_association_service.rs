use actix_web::error::ErrorNotFound;

use super::user_service;
use crate::{
    common::errors::{Result, ServiceError},
    pojo::dto::user::{UserAssociationDTO, UserProfileDTO},
    repository::user_association_repository,
};

pub async fn get_profile_by_idp_openid(idp_openid: &str) -> Result<UserProfileDTO> {
    let (openid, associations): (String, Vec<UserAssociationDTO>) =
        user_association_repository::select_user_associations_by_idp_openid(idp_openid).await?;
    match user_service::get_user(&openid, true).await? {
        Some(u) => {
            let mut user = UserProfileDTO::from(u);
            user.associations = associations.clone();
            Ok(user)
        }
        None => Err(ServiceError::ReponseError(ErrorNotFound("user not fount"))),
    }
}

pub async fn get_profile_with_associations(openid: &str) -> Result<UserProfileDTO> {
    match user_service::get_user(openid, true).await? {
        Some(u) => Ok(
            user_association_repository::batch_select_user_associations(&u.openid)
                .await
                .map(|associations| {
                    let mut user = UserProfileDTO::from(u);
                    user.associations = associations.clone();
                    user
                })?,
        ),
        None => Err(ServiceError::ReponseError(ErrorNotFound("user not fount"))),
    }
}

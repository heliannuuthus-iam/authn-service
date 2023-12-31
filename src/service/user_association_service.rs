use actix_web::error::ErrorNotFound;

use super::user_service;
use crate::{
    common::{
        datasource::{tx_begin, tx_commit},
        errors::{Result, ServiceError},
    },
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
        None => Err(ServiceError::Reponse(ErrorNotFound("user not fount"))),
    }
}

pub async fn get_profile_with_associations(openid: &str) -> Result<UserProfileDTO> {
    match user_service::get_user(openid, true).await? {
        Some(u) => Ok(
            user_association_repository::select_user_associations(&u.openid)
                .await
                .map(|associations| {
                    let mut user = UserProfileDTO::from(u);
                    user.associations = associations.clone();
                    user
                })?,
        ),
        None => Err(ServiceError::Reponse(ErrorNotFound("user not fount"))),
    }
}

pub async fn enstablish_idp_association(
    openid: &str,
    association: &UserAssociationDTO,
) -> Result<Vec<UserAssociationDTO>> {
    let mut tx = tx_begin("enstablish idp assos").await?;

    // table index determine to select instead of insert with duplicate error
    let mut associations = user_association_repository::select_user_associations(openid).await?;
    if associations
        .iter()
        .filter(|idp| !idp.idp_type.eq(&association.idp_type))
        .count()
        == 0
    {
        user_association_repository::create_associations(
            &mut tx,
            openid,
            &vec![association.clone()],
        )
        .await?;
        associations.push(association.clone());
    };
    tx_commit(tx, "enstablish idp assos").await?;
    Ok(associations)
}

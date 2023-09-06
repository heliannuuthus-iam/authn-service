use std::vec;

use chrono::Duration;
use serde_json::json;

use crate::{
    common::{
        cache::{cache_get, cache_setex},
        datasource::{tx_begin, tx_commit},
        enums::IdpType,
        errors::Result,
    },
    pojo::{
        dto::user::{UserAssociationDTO, UserProfileDTO},
        po::user::User,
    },
    repository::{
        user_association_repository::{self},
        user_repository::{self, select_profile},
    },
};

const USER_PROFILE_CACHE_KEY: &str = "forum:user:profile:";

pub async fn get_user(identifier: &str, is_openid: bool) -> Result<Option<User>> {
    if let Some(u) =
        cache_get::<User>(format!("{}{}", USER_PROFILE_CACHE_KEY, identifier).as_str()).await?
    {
        Ok(Some(u))
    } else {
        match if is_openid {
            select_profile(Some(identifier.to_string()), None)
        } else {
            select_profile(None, Some(identifier.to_string()))
        }
        .await?
        {
            Some(ref u) => {
                cache_setex(
                    format!("{}{}", USER_PROFILE_CACHE_KEY, identifier).as_str(),
                    u.clone(),
                    Duration::days(1),
                )
                .await?;
                Ok(Some(u.clone()))
            }
            None => Ok(None),
        }
    }
}

pub async fn create_user(email: &str, associations: &mut Vec<UserAssociationDTO>) -> Result<User> {
    let user = User::new(email);
    let tx = tx_begin("create user").await?;

    user_repository::save_user(&user).await?;
    associations.push(UserAssociationDTO::new(
        user.openid.clone(),
        IdpType::FORUM,
        json!(null),
    ));

    user_association_repository::batch_create_association(user.openid.as_str(), associations)
        .await?;

    tx_commit(tx, "create user").await?;
    Ok(user)
}

pub async fn enstablish_idp_association(
    email: &str,
    association: &UserAssociationDTO,
) -> Result<UserProfileDTO> {
    if let Some(user) = &get_user(email, false).await? {
        let tx = tx_begin("enstablish user association").await?;

        let basic = user_association_repository::select_user_association(
            &user.openid,
            association.idp_type,
            Some(association.idp_openid.to_string()),
        )
        .await?;

        let mut user_with_association: UserProfileDTO = user.clone().into();
        if let Some(uat) = basic {
            user_with_association.associations.push(uat);
        } else {
            let mut associations = vec![association.clone()];
            user_association_repository::batch_create_association(&user.openid, &mut associations)
                .await?;
            user_with_association.associations.push(association.clone());
        }

        tx_commit(tx, "enstablish user association").await?;
        Ok(user_with_association)
    } else {
        let mut associations = vec![association.clone()];
        let user = create_user(email, &mut associations).await?;
        let mut user_profile: UserProfileDTO = user.into();
        user_profile.associations.append(&mut associations);
        Ok(user_profile)
    }
}

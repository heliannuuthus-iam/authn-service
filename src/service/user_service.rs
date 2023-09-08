use std::vec;

use chrono::Duration;
use serde_json::json;

use crate::{
    common::{
        cache::{cache_get, cache_setex},
        datasource::CONN,
        enums::IdpType,
        errors::Result,
    },
    pojo::{dto::user::UserAssociationDTO, po::user::User},
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
                    Duration::minutes(30),
                )
                .await?;
                Ok(Some(u.clone()))
            }
            None => Ok(None),
        }
    }
}

pub async fn create_user(
    email: &str,
    association: Option<&UserAssociationDTO>,
) -> Result<(User, Vec<UserAssociationDTO>)> {
    let mut tx = CONN.begin().await.unwrap();
    let user = User::new(email);
    user_repository::save_user(&user).await?;
    let mut inserted = vec![UserAssociationDTO::new(
        user.openid.clone(),
        IdpType::Forum,
        json!(null),
    )];
    if let Some(asso) = association {
        inserted.push(asso.clone());
    };
    user_association_repository::create_associations(tx.as_mut(), user.openid.as_str(), &inserted)
        .await?;
    tx.commit().await.unwrap();
    Ok((user, inserted))
}

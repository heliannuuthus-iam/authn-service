use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    common::enums::IdpType,
    pojo::po::user::{User, UserAssociation},
};

#[derive(Serialize, Deserialize, Debug, Clone, Builder, ToSchema)]
pub struct UserProfileDTO {
    pub openid: String,
    pub email: String,
    pub avatar: String,
    pub gender: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub associations: Vec<UserAssociationDTO>,
}

impl From<User> for UserProfileDTO {
    fn from(value: User) -> Self {
        UserProfileDTOBuilder::default()
            .openid(value.openid)
            .email(value.email)
            .avatar(value.avatar)
            .gender(value.gander)
            .associations(vec![])
            .build()
            .unwrap()
    }
}

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, Clone, ToSchema)]
pub struct UserAssociationDTO {
    pub idp_openid: String,
    pub idp_type: IdpType,
    #[serde(skip_serializing_if = "serde_json::Value::is_null")]
    pub idp_extra: serde_json::Value,
}

impl UserAssociationDTO {
    pub fn new(idp_openid: String, idp_type: IdpType, idp_extra: serde_json::Value) -> Self {
        Self {
            idp_openid,
            idp_type,
            idp_extra,
        }
    }
}

impl From<UserAssociation> for UserAssociationDTO {
    fn from(value: UserAssociation) -> Self {
        Self {
            idp_openid: value.idp_openid,
            idp_type: value.idp_type,
            idp_extra: value.idp_extra,
        }
    }
}

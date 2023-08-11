use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    common::enums::IdpType,
    pojo::po::user::{User, UserAssociation},
};

#[derive(Serialize, Deserialize, Clone, Builder, ToSchema)]
pub struct UserProfileDTO {
    pub openid: String,
    pub email: String,
    pub avatar: String,
    pub gender: String,
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

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct UserAssociationDTO {
    #[serde(rename = "idp_openid")]
    pub idp_openid: String,

    #[serde(rename = "idp_type")]
    pub idp_type: IdpType,
}

impl From<UserAssociation> for UserAssociationDTO {
    fn from(value: UserAssociation) -> Self {
        Self {
            idp_openid: value.idp_openid,
            idp_type: value.idp_type,
        }
    }
}

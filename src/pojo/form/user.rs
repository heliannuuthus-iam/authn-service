use std::fmt::Debug;

use utoipa::{IntoParams, ToSchema};

use crate::{
    common::utils::{desensitize_email, desensitize_text},
    pojo::dto::user::UserAssociationDTO,
};

#[derive(serde::Serialize, serde::Deserialize, ToSchema, IntoParams)]
pub struct SrpPasswordForm {
    pub identifier: String,
    pub verifier: String,
    pub salt: String,
}

impl Debug for SrpPasswordForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SrpPasswordForm")
            .field("identifier", &desensitize_email(&self.identifier))
            .field("verifier", &desensitize_text(&self.verifier))
            .field("salt", &desensitize_text(&self.salt))
            .finish()
    }
}

#[derive(serde::Serialize, serde::Deserialize, ToSchema, IntoParams)]
pub struct UserAssoInitialForm {
    pub identifier: String,
    pub association: UserAssociationDTO,
}

impl Debug for UserAssoInitialForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UserAssoInitialForm")
            .field("identifier", &desensitize_email(&self.identifier))
            .field("association", &self.association)
            .finish()
    }
}

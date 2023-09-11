use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::common::{config::env_var_default, enums::IdpType, utils::gen_id};
#[derive(Serialize, Deserialize, Clone, sqlx::FromRow, Default, ToSchema)]
pub struct User {
    pub openid: String,
    pub avatar: String,
    pub username: String,
    pub gander: String,
    pub email: String,
    pub email_verified: bool,
    pub registried_at: DateTime<Utc>,
}

impl User {
    pub fn new(email: &str) -> Self {
        User {
            openid: gen_id(22),
            username: gen_id(32),
            email: email.to_string(),
            avatar: env_var_default::<String>(
                "DEFAULT_IMAGE",
                String::from("https://docs.rs/-/rustdoc.static/rust-logo-151179464ae7ed46.svg"),
            ),
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow, Clone, ToSchema)]
pub struct UserAssociation {
    pub openid: String,
    pub idp_openid: String,
    pub idp_type: IdpType,
    pub idp_extra: serde_json::Value,
}

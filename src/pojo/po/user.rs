use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::common::enums::IdpType;
#[derive(Serialize, Deserialize, Clone, sqlx::FromRow, ToSchema)]
pub struct User {
    pub openid: String,
    pub avatar: String,
    pub username: String,
    pub gander: String,
    pub email: String,
    pub email_verified: bool,
    pub registried_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow, Clone, ToSchema)]
pub struct UserAssociation {
    pub openid: String,
    pub idp_openid: String,
    pub idp_type: IdpType,
}

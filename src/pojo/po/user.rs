use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::common::enums::IdpType;
#[derive(Serialize, Deserialize, Clone, sqlx::FromRow, ToSchema)]
pub struct User {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "openid")]
    pub openid: String,

    #[serde(rename = "avatar")]
    pub avatar: String,

    #[serde(rename = "username")]
    pub username: String,

    #[serde(rename = "gander")]
    pub gander: String,

    #[serde(rename = "email")]
    pub email: String,

    #[serde(rename = "email_verified")]
    pub email_verified: bool,

    #[serde(rename = "registered_at")]
    pub registered_at: DateTime<Utc>,

    #[serde(rename = "updated_at")]
    pub updated_at: DateTime<Utc>,

    #[serde(rename = "created_at")]
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct UserAssociation {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "openid")]
    pub openid: String,

    #[serde(rename = "idp_openid")]
    pub idp_openid: String,

    #[serde(rename = "idp_type")]
    pub idp_type: IdpType,

    #[serde(rename = "updated_at")]
    pub updated_at: DateTime<Utc>,

    #[serde(rename = "created_at")]
    pub created_at: DateTime<Utc>,
}

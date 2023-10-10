use std::str::FromStr;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Deserialize,
    Serialize,
    PartialEq,
    Eq,
    Debug,
    Clone,
    Copy,
    Default,
    ToSchema,
    sqlx::Encode,
    sqlx::Decode,
)]
#[sqlx(type_name = "idp_type", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum IdpType {
    GitHub,
    Google,
    #[default]
    User,
}

impl sqlx::Type<sqlx::MySql> for IdpType {
    fn type_info() -> <sqlx::MySql as sqlx::Database>::TypeInfo {
        <str as sqlx::Type<sqlx::MySql>>::type_info()
    }

    fn compatible(ty: &<sqlx::MySql as sqlx::Database>::TypeInfo) -> bool {
        <str as sqlx::Type<sqlx::MySql>>::compatible(ty)
    }
}

impl FromStr for IdpType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "github" {
            Ok(IdpType::GitHub)
        } else if s == "google" {
            Ok(IdpType::Google)
        } else {
            Ok(IdpType::User)
        }
    }
}

impl From<String> for IdpType {
    fn from(value: String) -> Self {
        if value == "github" {
            IdpType::GitHub
        } else if value == "google" {
            IdpType::Google
        } else {
            IdpType::User
        }
    }
}

#[derive(
    Deserialize,
    Serialize,
    PartialEq,
    Eq,
    Debug,
    Clone,
    Copy,
    Default,
    ToSchema,
    sqlx::Encode,
    sqlx::Decode,
)]
#[sqlx(type_name = "client_type", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ClientType {
    #[default]
    #[sqlx(rename = "0")]
    MTM, // machine to machine
    #[sqlx(rename = "1")]
    WEB, // singal page application and regular web application
}

impl From<i8> for ClientType {
    fn from(value: i8) -> Self {
        if value == 0 {
            Self::MTM
        } else {
            Self::WEB
        }
    }
}

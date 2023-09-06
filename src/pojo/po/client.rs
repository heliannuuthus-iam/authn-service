use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::common::{datasource::to_vec, enums::IdpType};

#[derive(Serialize, Deserialize, sqlx::FromRow, Default, ToSchema)]
pub struct ClientConfig {
    pub client_id: String,
    pub name: String,
    pub logo: String,
    pub description: Option<String>,
    #[serde(serialize_with = "to_vec")]
    pub redirect_url: Option<String>,
}

#[derive(Serialize, Deserialize, Default, sqlx::FromRow, ToSchema)]
pub struct ClientIdpConfig {
    pub client_id: String,
    pub idp_type: IdpType,
    pub idp_client_id: String,
    pub idp_client_secret: String,
}

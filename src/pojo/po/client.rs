use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::common::{
    datasource::to_vec,
    enums::{ClientType, IdpType},
};

#[derive(Serialize, Deserialize, sqlx::FromRow, Default, ToSchema, Debug)]
pub struct Client {
    pub client_id: String,
    pub name: String,
    pub logo: String,
    pub description: Option<String>,
    pub client_type: ClientType,
}

#[derive(Serialize, Deserialize, sqlx::FromRow, Default, ToSchema, Debug)]
pub struct ClientSetting {
    #[sqlx(skip)]
    #[serde(flatten)]
    pub client: Client,
    pub client_id: String,
    #[serde(serialize_with = "to_vec")]
    pub callbacks: Option<String>,
    #[serde(serialize_with = "to_vec")]
    pub allowed_origins: Option<String>,
    #[serde(serialize_with = "to_vec")]
    pub allowed_logout_urls: Option<String>,
}

#[derive(Serialize, Deserialize, Default, sqlx::FromRow, ToSchema)]
pub struct ClientIdpConfig {
    pub idp_type: IdpType,
    pub idp_client_id: String,
    pub idp_client_secret: String,
}

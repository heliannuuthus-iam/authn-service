use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    common::{datasource::to_vec, enums::IdpType, utils::gen_id},
    pojo::form::client::{ClientConfigCreateForm, ClientIdpConfigSaveOrUpdateForm},
};

#[derive(Serialize, Deserialize, sqlx::FromRow, Default, ToSchema)]
pub struct ClientConfig {
    pub client_id: String,
    pub name: String,
    pub logo: String,
    pub description: Option<String>,
    #[serde(serialize_with = "to_vec")]
    pub redirect_url: Option<String>,
}

impl From<ClientConfigCreateForm> for ClientConfig {
    fn from(val: ClientConfigCreateForm) -> Self {
        ClientConfig {
            client_id: gen_id(32),
            name: val.name,
            logo: val.logo,
            description: val.desc,
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Default, sqlx::FromRow, ToSchema)]
pub struct ClientIdpConfig {
    pub client_id: String,
    pub idp_type: IdpType,
    pub idp_client_id: String,
    pub idp_client_secret: String,
}

impl From<ClientIdpConfigSaveOrUpdateForm> for ClientIdpConfig {
    fn from(val: ClientIdpConfigSaveOrUpdateForm) -> Self {
        ClientIdpConfig {
            idp_client_id: val.idp_client_id,
            idp_type: val.idp_type,
            idp_client_secret: val.idp_client_secret,
            ..Default::default()
        }
    }
}

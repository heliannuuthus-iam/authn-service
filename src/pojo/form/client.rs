use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::{
    common::{
        config::Patch,
        datasource::from_vec,
        enums::{ClientType, IdpType},
        utils::gen_id,
    },
    pojo::po::client::{Client, ClientIdpConfig, ClientSetting},
};

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct ClientSettingCreateForm {
    pub name: String,
    pub logo: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    pub client_type: ClientType,
}

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct ClientSettingUpdateForm {
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_type: Option<ClientType>,
    #[serde(deserialize_with = "from_vec")]
    pub callbacks: Option<String>,
    #[serde(deserialize_with = "from_vec")]
    pub allowed_origins: Option<String>,
    #[serde(deserialize_with = "from_vec")]
    pub allowed_logout_urls: Option<String>,
}

impl From<ClientSettingCreateForm> for ClientSetting {
    fn from(val: ClientSettingCreateForm) -> Self {
        let client_id: &str = &gen_id(32);
        ClientSetting {
            client: Client {
                client_id: client_id.to_string(),
                name: val.name,
                logo: val.logo,
                description: val.desc,
                client_type: val.client_type,
            },
            client_id: client_id.to_string(),
            ..Default::default()
        }
    }
}

impl Patch for ClientSettingUpdateForm {
    type Into = ClientSetting;

    fn merge(&self, into: &mut Self::Into) {
        if self.name.is_some() {
            into.client.name = self.name.clone().unwrap();
        }
        if self.logo.is_some() {
            into.client.logo = self.logo.clone().unwrap();
        }
        if self.desc.is_some() {
            into.client.description = self.desc.clone();
        }
    }
}

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct ClientIdpConfigSaveOrUpdateForm {
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
        }
    }
}

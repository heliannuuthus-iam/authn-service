use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::{
    common::{config::Patch, datasource::from_vec, enums::IdpType, utils::gen_id},
    pojo::po::client::{ClientConfig, ClientIdpConfig},
};

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct ClientConfigCreateForm {
    pub name: String,
    pub logo: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
}

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct ClientConfigUpdateForm {
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(deserialize_with = "from_vec")]
    pub redirect_url: Option<String>,
}

impl Patch for ClientConfigUpdateForm {
    type Into = ClientConfig;

    fn merge(&self, into: &mut Self::Into) {
        if self.name.is_some() {
            into.name = self.name.clone().unwrap();
        }
        if self.logo.is_some() {
            into.logo = self.logo.clone().unwrap();
        }
        if self.desc.is_some() {
            into.description = self.desc.clone();
        }
        if self.redirect_url.is_some() {
            into.redirect_url = self.redirect_url.clone();
        }
    }
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
            ..Default::default()
        }
    }
}

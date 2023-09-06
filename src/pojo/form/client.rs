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

impl Into<ClientConfig> for ClientConfigCreateForm {
    fn into(self) -> ClientConfig {
        ClientConfig {
            client_id: gen_id(32),
            name: self.name,
            logo: self.logo,
            description: self.desc,
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

impl Into<ClientIdpConfig> for ClientIdpConfigSaveOrUpdateForm {
    fn into(self) -> ClientIdpConfig {
        ClientIdpConfig {
            idp_client_id: self.idp_client_id,
            idp_type: self.idp_type,
            idp_client_secret: self.idp_client_secret,
            ..Default::default()
        }
    }
}

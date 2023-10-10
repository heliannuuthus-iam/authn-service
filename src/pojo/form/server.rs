use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::{
    common::{config::Patch, utils::gen_id},
    pojo::po::server::{Server, ServerSetting},
};

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct ServerSettingCreateForm {
    pub name: String,
    pub logo: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
}

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct ServerSettingUpdateForm {
    pub server_id: String,
    pub server: Server,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    pub allow_offline_access: Option<bool>,
    pub token_lifttime: Option<u64>,
    pub signing_alg: Option<String>,
}

impl From<ServerSettingCreateForm> for ServerSetting {
    fn from(val: ServerSettingCreateForm) -> Self {
        let server_id: &str = &gen_id(32);
        ServerSetting {
            server: Server {
                server_id: server_id.to_string(),
                name: val.name,
                logo: val.logo,
                description: val.desc,
            },
            server_id: server_id.to_string(),
            ..Default::default()
        }
    }
}

impl Patch for ServerSettingUpdateForm {
    type Into = ServerSetting;

    fn merge(&self, into: &mut Self::Into) {
        if self.name.is_some() {
            into.server.name = self.name.clone().unwrap();
        }
        if self.logo.is_some() {
            into.server.logo = self.logo.clone().unwrap();
        }
        if self.desc.is_some() {
            into.server.description = self.desc.clone();
        }
        if self.allow_offline_access.is_none() {
            into.allow_offline_access = self.allow_offline_access.unwrap();
        }
        if self.token_lifttime.is_none() {
            into.token_lifttime = self.token_lifttime.unwrap();
        }
        if self.signing_alg.is_none() {
            into.signing_alg = self.signing_alg.clone().unwrap();
        }
    }
}

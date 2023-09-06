use utoipa::{IntoParams, ToSchema};

#[derive(serde::Serialize, serde::Deserialize, Debug, ToSchema, IntoParams)]
pub struct RegistryForm {
    pub identifier: String,
    pub verifier: String,
    pub salt: String,
}

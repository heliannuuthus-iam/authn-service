use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Client {
    id: String,
    #[serde(rename = "client_id")]
    client_id: String,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "logo")]
    logo: String,

    #[serde(rename = "description")]
    description: String,

    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct ClientConfig {
    id: String,
    #[serde(rename = "client_id")]
    client_id: String,

    #[serde(rename = "redirect_uri")]
    redirect_uri: String,

    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct ClientIdpConfig {
    id: String,
    #[serde(rename = "client_id")]
    client_id: String,

    #[serde(rename = "idp_type")]
    idp_type: String,

    #[serde(rename = "idp_client_id")]
    idp_client_id: String,

    #[serde(rename = "idp_client_secret")]
    idp_client_secret: String,

    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

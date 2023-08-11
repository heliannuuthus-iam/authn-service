use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, sqlx::FromRow, Default, ToSchema)]
pub struct ClientConfig {
    pub id: i64,
    pub client_id: String,
    pub name: String,
    pub logo: String,
    pub description: String,
    pub redirect_url: Option<String>,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ClientIdpConfig {
    pub id: i64,
    pub client_id: String,
    pub idp_type: String,
    pub idp_client_id: String,
    pub idp_client_secret: String,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

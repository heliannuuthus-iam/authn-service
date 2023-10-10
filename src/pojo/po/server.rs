use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, sqlx::FromRow, Default, ToSchema, Debug)]
pub struct Server {
    pub server_id: String,
    pub name: String,
    pub logo: String,
    pub description: Option<String>,
}
#[derive(Serialize, Deserialize, sqlx::FromRow, Default, ToSchema, Debug)]
pub struct ServerSetting {
    #[sqlx(skip)]
    #[serde(flatten)]
    pub server: Server,
    pub server_id: String,
    pub allow_offline_access: bool,
    pub token_lifttime: u64,
    pub signing_alg: String,
}

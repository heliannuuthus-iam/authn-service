use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, sqlx::FromRow, ToSchema, Debug)]
pub struct ChallengeCofig {
    pub client_id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub challenge_type: String,
}

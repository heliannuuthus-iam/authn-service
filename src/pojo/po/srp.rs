use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, sqlx::FromRow, ToSchema)]
pub struct SrpPassword {
    pub identifier: String,
    pub verifier: String,
    pub salt: String,
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, sqlx::FromRow, ToSchema)]
pub struct SmsConfig {
    pub name: String,
    pub template: String,
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, sqlx::FromRow, ToSchema)]
pub struct SmsConfig {
    pub id: i64,
    pub name: String,
    pub template: String,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

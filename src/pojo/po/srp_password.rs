use std::fmt::{Debug, Formatter};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SrpPassword {
    pub id: i64,
    pub identifier: String,
    pub verifier: String,
    pub salt: String,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl Debug for SrpPassword {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SrpPassword")
            .field("id", &self.id)
            .field("updated_at", &self.updated_at)
            .field("created_at", &self.created_at)
            .finish()
    }
}

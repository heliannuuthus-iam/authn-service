use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Clone, Copy, Default, ToSchema)]
pub enum IdpType {
    #[serde(rename = "github")]
    GitHub,
    #[serde(rename = "google")]
    Google,
    #[serde(skip)]
    #[default]
    FORUM,
}

impl From<String> for IdpType {
    fn from(value: String) -> Self {
        if value == "github" {
            IdpType::GitHub
        } else if value == "google" {
            IdpType::Google
        } else {
            IdpType::FORUM
        }
    }
}

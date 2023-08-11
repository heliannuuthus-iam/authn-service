use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Copy, Default)]
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

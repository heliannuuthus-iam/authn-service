use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Clone, Copy)]
pub enum IdpType {
    #[serde(rename = "github")]
    GitHub,
    #[serde(rename = "google")]
    Google,
    #[serde(rename = "wechat")]
    Wechat,
    #[serde(rename = "tencent")]
    Tencent,
}

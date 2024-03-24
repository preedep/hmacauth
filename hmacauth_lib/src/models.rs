use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    #[serde(rename = "message")]
    pub message: Option<String>,
}
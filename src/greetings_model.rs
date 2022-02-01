use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GreetingsModel {
    #[serde(rename = "message")]
    pub message: String,
}

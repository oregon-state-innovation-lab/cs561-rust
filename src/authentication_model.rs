use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationModel {
    #[serde(rename = "access-token")]
    pub access_token: String,

    #[serde(rename = "expires")]
    pub expires: String,
}

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetToken {
    #[serde(rename = "access_token")]
    pub access_token: String,
    #[serde(rename = "token_type")]
    pub token_type: String,
    #[serde(rename = "expires_in")]
    pub expires_in: i64,
}

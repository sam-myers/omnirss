use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetShow {
    #[serde(rename = "available_markets")]
    pub available_markets: Vec<String>,
    pub copyrights: Vec<Value>,
    pub description: String,
    pub episodes: Episodes,
    pub explicit: bool,
    #[serde(rename = "external_urls")]
    pub external_urls: ExternalUrls2,
    pub href: String,
    #[serde(rename = "html_description")]
    pub html_description: String,
    pub id: String,
    pub images: Vec<Image2>,
    #[serde(rename = "is_externally_hosted")]
    pub is_externally_hosted: bool,
    pub languages: Vec<String>,
    #[serde(rename = "media_type")]
    pub media_type: String,
    pub name: String,
    pub publisher: String,
    #[serde(rename = "total_episodes")]
    pub total_episodes: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Episodes {
    pub href: String,
    pub items: Vec<Item>,
    pub limit: i64,
    pub next: String,
    pub offset: i64,
    pub previous: Value,
    pub total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    #[serde(rename = "audio_preview_url")]
    pub audio_preview_url: String,
    pub description: String,
    #[serde(rename = "duration_ms")]
    pub duration_ms: i64,
    pub explicit: bool,
    #[serde(rename = "external_urls")]
    pub external_urls: ExternalUrls,
    pub href: String,
    #[serde(rename = "html_description")]
    pub html_description: String,
    pub id: String,
    pub images: Vec<Image>,
    #[serde(rename = "is_externally_hosted")]
    pub is_externally_hosted: bool,
    #[serde(rename = "is_playable")]
    pub is_playable: bool,
    pub language: String,
    pub languages: Vec<String>,
    pub name: String,
    #[serde(rename = "release_date")]
    pub release_date: String,
    #[serde(rename = "release_date_precision")]
    pub release_date_precision: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalUrls {
    pub spotify: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub height: i64,
    pub url: String,
    pub width: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalUrls2 {
    pub spotify: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image2 {
    pub height: i64,
    pub url: String,
    pub width: i64,
}

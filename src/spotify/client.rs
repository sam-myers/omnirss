use std::ops::Add;
use base64::encode;
use chrono::prelude::*;
use chrono::Duration;
use log::{trace, debug};

use crate::spotify::credentials::SpotifyCredentials;
use crate::spotify::response;
use crate::error::*;

const BASE_URL: &str = "https://api.spotify.com/v1";
const GRANT_TYPE: &str = "client_credentials";

pub struct SpotifyClient {
    credentials: SpotifyCredentials,
    token: SpotifyToken,
    token_expiry: DateTime<Utc>,
}

struct SpotifyToken(String);

impl SpotifyToken {
    pub fn bearer_auth_header(&self) -> String {
        format!("Bearer {}", self.0)
    }
}

impl SpotifyClient {
    pub async fn new() -> Result<Self> {
        let credentials = SpotifyCredentials::from_env_vars()?;
        let resp = Self::get_token(&credentials).await?;

        Ok(Self {
            credentials,
            token: SpotifyToken(resp.access_token),
            token_expiry: Utc::now().add(Duration::seconds(resp.expires_in)),
        })
    }

    async fn get_token(credentials: &SpotifyCredentials) -> Result<response::GetToken> {
        debug!("Spotify client getting debug token");

        let resp = reqwest::Client::new()
            .post("https://accounts.spotify.com/api/token")
            .header("Authorization", credentials.basic_auth_header())
            .form(&[("grant_type", "client_credentials")])
            .send()
            .await?
            .json::<response::GetToken>()
            .await?;
        trace!("Token response {:?}", resp);
        Ok(resp)
    }

    async fn get<T: for <'a> serde::Deserialize<'a> + std::fmt::Debug>(&self, path: String) -> Result<T> {
        let resp = reqwest::Client::new()
            .get(format!("{}/{}", BASE_URL, path))
            .header("Authorization", self.token.bearer_auth_header())
            .query(&[("market", "US")])
            .send()
            .await?
            .json::<T>()
            .await?;
        trace!("Get response {:?}", resp);
        Ok(resp)
    }

    pub async fn get_shows(&self, show_id: String) -> Result<response::GetShow> {
        debug!("Getting Spotify show id {}", show_id);
        self.get::<response::GetShow>(format!("shows/{}", show_id)).await
    }
}

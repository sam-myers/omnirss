use chrono::prelude::*;
use chrono::Duration;
use futures::lock::{Mutex, MutexGuard};
use log::{debug, trace};
use std::ops::Add;

use crate::error::*;
use crate::settings::Settings;
use crate::spotify::credentials::SpotifyCredentials;
use crate::spotify::response;

const BASE_URL: &str = "https://api.spotify.com/v1";

pub struct SpotifyClient {
    credentials: SpotifyCredentials,
    token: Mutex<SpotifyToken>,
}

struct SpotifyToken {
    token: String,
    expiry: DateTime<Utc>,
}

impl SpotifyToken {
    pub fn bearer_auth_header(&self) -> String {
        format!("Bearer {}", self.token)
    }
}

impl SpotifyClient {
    pub async fn from_config(config: &Settings) -> Result<Self> {
        let credentials = SpotifyCredentials::from_config(config)?;
        let resp = Self::request_token(&credentials).await?;

        Ok(Self {
            credentials,
            token: Mutex::new(SpotifyToken {
                token: resp.access_token,
                expiry: Utc::now().add(Duration::seconds(resp.expires_in)),
            }),
        })
    }

    async fn request_token(credentials: &SpotifyCredentials) -> Result<response::GetToken> {
        debug!("Spotify client getting access token");

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

    async fn refresh_token_if_needed(&self) -> Result<()> {
        let mut current_token: MutexGuard<SpotifyToken> = self.token.lock().await;
        if current_token
            .expiry
            .signed_duration_since(Utc::now())
            .num_seconds()
            < 0
        {
            info!("Refreshing Spotify token");
            let new_token_response = SpotifyClient::request_token(&self.credentials).await?;
            *current_token = SpotifyToken {
                token: new_token_response.access_token,
                expiry: Utc::now().add(Duration::seconds(new_token_response.expires_in)),
            };
        }
        Ok(())
    }

    async fn get_bearer_auth_header(&self) -> Result<String> {
        self.refresh_token_if_needed().await?;
        Ok(self.token.lock().await.bearer_auth_header())
    }

    pub async fn get_shows(&self, show_id: &str) -> Result<response::GetShow> {
        debug!("Getting Spotify show id {}", show_id);
        let resp = reqwest::Client::new()
            .get(format!("{}/shows/{}", BASE_URL, show_id))
            .header("Authorization", self.get_bearer_auth_header().await?)
            .query(&[("market", "US")])
            .send()
            .await?
            .json::<response::GetShow>()
            .await?;
        Ok(resp)
    }

    // pub async fn search_shows(&self, query: &String) -> Result<response::GetShow> {
    //     debug!("Searching for show with query {}", query);
    //     let resp = reqwest::Client::new()
    //         .get(format!("{}/search", BASE_URL))
    //         .header("Authorization", self.get_bearer_auth_header().await?)
    //         .query(&[("market", "US"), ("type", "show"), ("q", query)])
    //         .send()
    //         .await?
    //         .json::<response::GetShow>()
    //         .await?;
    //     Ok(resp)
    // }
}

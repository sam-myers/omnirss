use chrono::prelude::*;
use chrono::Duration;
use futures::lock::{Mutex, MutexGuard};
use log::{debug, trace};
use std::ops::Add;

use crate::error::*;
use crate::settings::Settings;
use crate::spotify::credentials::SpotifyCredentials;
use crate::spotify::token::SpotifyToken;
use crate::spotify::{response, Spotify};

const BASE_URL: &str = "https://api.spotify.com/v1";

pub struct SpotifyClient {
    credentials: SpotifyCredentials,
    token: Mutex<SpotifyToken>,
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

    async fn _get_shows(&self, show_id: &str) -> Result<Box<response::GetShow>> {
        debug!("Getting Spotify show id {}", show_id);
        let resp = reqwest::Client::new()
            .get(format!("{}/shows/{}", BASE_URL, show_id))
            .header("Authorization", self.get_bearer_auth_header().await?)
            .query(&[("market", "US")])
            .send()
            .await?
            .json::<Box<response::GetShow>>()
            .await?;
        Ok(resp)
    }

    async fn _search_show(&self, query: &str) -> Result<Box<response::Search>> {
        debug!("Searching Spotify for show {}", query);
        let resp = reqwest::Client::new()
            .get(format!("{}/search", BASE_URL))
            .header("Authorization", self.get_bearer_auth_header().await?)
            .query(&[("market", "US"), ("type", "show"), ("q", query)])
            .send()
            .await?
            .json::<Box<response::Search>>()
            .await?;
        Ok(resp)
    }
}

#[rocket::async_trait]
impl Spotify for SpotifyClient {
    async fn get_show(&self, show_id: &str) -> Result<Box<response::GetShow>> {
        self._get_shows(show_id).await
    }

    async fn search_show(&self, query: &str) -> Result<Box<response::Search>> {
        self._search_show(query).await
    }
}

#[rocket::async_trait]
impl<'a> Spotify for &'a SpotifyClient {
    async fn get_show(&self, show_id: &str) -> Result<Box<response::GetShow>> {
        self._get_shows(show_id).await
    }

    async fn search_show(&self, query: &str) -> Result<Box<response::Search>> {
        self._search_show(query).await
    }
}

mod cache;
mod client;
mod credentials;
mod mock;
mod response;
mod service;
mod token;

pub use self::service::SpotifyService;
use crate::error::Result;
pub use client::SpotifyClient;
pub use mock::MockSpotifyClient;

#[rocket::async_trait]
pub trait Spotify {
    async fn get_show(&self, show_id: &str) -> Result<Box<response::GetShow>>;
    async fn search_show(&self, query: &str) -> Result<Box<response::Search>>;
}

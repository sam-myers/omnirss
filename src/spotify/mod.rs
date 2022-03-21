mod cache;
mod client;
mod credentials;
mod response;
mod rss;
mod token;

pub use self::rss::SpotifyRss;
use crate::error::Result;
pub use client::SpotifyClient;

#[rocket::async_trait]
pub trait Spotify {
    async fn get_shows(&self, show_id: &str) -> Result<response::GetShow>;
}

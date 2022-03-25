mod cache;
mod client;
mod credentials;
mod mock;
mod response;
mod rss;
mod token;

pub use self::rss::SpotifyRss;
use crate::error::Result;
pub use client::SpotifyClient;
pub use mock::MockSpotifyClient;

#[rocket::async_trait]
pub trait Spotify {
    async fn get_shows(&self, show_id: &str) -> Result<Box<response::GetShow>>;
}

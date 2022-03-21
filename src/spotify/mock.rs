use crate::error::{OmniRssError, Result};
use crate::spotify::{response, Spotify};

pub struct MockSpotifyClient {}

#[rocket::async_trait]
impl Spotify for MockSpotifyClient {
    async fn get_shows(&self, show_id: &str) -> Result<response::GetShow> {
        match show_id {
            "4rOoJ6Egrf8K2IrywzwOMk" => Ok(response::GetShow::test_fixture_joe_rogan()),
            _ => Err(OmniRssError::MockSpotifyNotImplemented(
                "show id not mocked",
            )),
        }
    }
}

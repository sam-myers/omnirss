use crate::error::{OmniRssError, Result};
use crate::spotify::response::Search;
use crate::spotify::{response, Spotify};

pub struct MockSpotifyClient {}

#[rocket::async_trait]
impl Spotify for MockSpotifyClient {
    async fn get_show(&self, show_id: &str) -> Result<Box<response::GetShow>> {
        match show_id {
            "4rOoJ6Egrf8K2IrywzwOMk" => Ok(response::GetShow::test_fixture_joe_rogan()),
            _ => Err(OmniRssError::MockSpotifyNotImplemented(
                "show id not mocked",
            )),
        }
    }

    async fn search_show(&self, query: &str) -> Result<Box<Search>> {
        match query {
            "joe" => Ok(response::Search::test_fixture_search_joe()),
            _ => Err(OmniRssError::MockSpotifyNotImplemented("search not mocked")),
        }
    }
}

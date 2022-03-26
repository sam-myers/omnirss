use crate::cache::{Cache, RedisCache};
use crate::spotify::{SpotifyClient, SpotifyRss};
use rocket::{Route, State};

#[get("/health")]
async fn health(redis_client: &State<RedisCache>) -> Option<&'static str> {
    if !redis_client.inner().ping().await {
        return None;
    }
    Some("OK")
}

#[get("/spotify/id/<show_id>")]
async fn spotify_by_id(
    show_id: String,
    spotify_client: &State<SpotifyClient>,
    cache: &State<RedisCache>,
) -> Option<String> {
    SpotifyRss::show_feed(spotify_client.inner(), cache.inner(), show_id)
        .await
        .ok()
}

pub fn routes() -> Vec<Route> {
    routes![health, spotify_by_id]
}

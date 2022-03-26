use crate::cache::{Cache, RedisCache};
use crate::search_query::SearchQuery;
use crate::spotify::{SpotifyClient, SpotifyService};
use rocket::{Route, State};
use rocket_dyn_templates::Template;

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
    SpotifyService::show_feed(spotify_client.inner(), cache.inner(), show_id)
        .await
        .ok()
}

#[get("/?<search>")]
async fn search(
    search: SearchQuery,
    spotify_client: &State<SpotifyClient>,
    cache: &State<RedisCache>,
) -> Option<Template> {
    let context = SpotifyService::search_show(spotify_client.inner(), cache.inner(), search)
        .await
        .ok()?;

    Some(Template::render("search-results", &context))
}

pub fn routes() -> Vec<Route> {
    routes![health, spotify_by_id, search]
}

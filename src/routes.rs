use crate::cache::{Cache, RedisCache};
use crate::search_query::SearchQuery;
use crate::settings::Settings;
use crate::spotify::{SpotifyClient, SpotifyService};
use rocket::{Route, State};
use rocket_dyn_templates::Template;
use tracing::error;

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
    match SpotifyService::show_feed(spotify_client.inner(), cache.inner(), show_id).await {
        Ok(result) => Some(result),
        Err(e) => {
            error!("Getting feed: {:?}", e);
            None
        }
    }
}

#[get("/?<search>")]
async fn search(
    search: SearchQuery,
    spotify_client: &State<SpotifyClient>,
    cache: &State<RedisCache>,
    settings: &State<Settings>,
) -> Option<Template> {
    let context =
        match SpotifyService::search_show(spotify_client.inner(), cache.inner(), search, settings)
            .await
        {
            Ok(result) => Some(result),
            Err(e) => {
                error!("Searching for show: {:?}", e);
                None
            }
        };

    Some(Template::render("search-results", &context))
}

pub fn routes() -> Vec<Route> {
    routes![health, spotify_by_id, search]
}

use crate::cache::{Cache, RedisCache};
use crate::search_query::SearchQuery;
use crate::spotify::{SpotifyClient, SpotifyService};
use rocket::{Route, State};
use rocket_dyn_templates::Template;
use crate::response::{SearchResults, SearchResult};

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
async fn search(search: SearchQuery<'_>) -> Template {
    let context = SearchResults{ query: search.0.to_string(), results: vec![SearchResult{
        name: "Joe Rogan".to_string(),
        description: "foo bar biz baz".to_string(),
        image_url: "https://user-images.githubusercontent.com/5410234/160204775-f5efb737-ce6a-4698-a603-6bc159f56608.png".to_string()
    }] };
    Template::render("search-results", &context)
}

pub fn routes() -> Vec<Route> {
    routes![health, spotify_by_id, search]
}

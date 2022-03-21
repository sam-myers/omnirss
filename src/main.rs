#[macro_use]
extern crate rocket;
extern crate log;
extern crate pretty_env_logger;
extern crate redis;
#[macro_use]
extern crate serde_derive;

use crate::cache::*;
use crate::spotify::*;

mod cache;
mod error;
mod settings;
mod spotify;

use log::info;
use rocket::State;
use rocket_dyn_templates::Template;
use std::collections::HashMap;

#[get("/")]
fn index() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("index", &context)
}

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

#[rocket::main]
#[allow(unused_must_use)]
async fn main() {
    // Config
    let rocket_builder = rocket::build();
    let config: settings::Settings = rocket_builder.figment().extract().expect("config");

    // Cache
    let redis_cache = RedisCache::from_settings(&config).expect("redis client");
    if !(&redis_cache).ping().await {
        panic!("Failed Redis health check");
    }

    // Spotify API
    info!("Starting Spotify client");
    let spotify_client = spotify::SpotifyClient::from_config(&config)
        .await
        .expect("spotify client");

    info!("Starting server");
    rocket_builder
        .manage(redis_cache)
        .manage(spotify_client)
        .mount("/", routes![index, health, spotify_by_id])
        .attach(Template::fairing())
        .launch()
        .await;
}

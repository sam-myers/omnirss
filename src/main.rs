#[macro_use]
extern crate rocket;
extern crate log;
extern crate pretty_env_logger;
extern crate redis;
#[macro_use]
extern crate serde_derive;

use crate::spotify::*;

mod cache;
mod error;
mod settings;
mod spotify;

use cache::Ping;
use log::info;
use rocket::State;

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

#[get("/health")]
async fn health(redis_client: &State<redis::Client>) -> Option<&'static str> {
    if !redis_client.ping().await {
        return None;
    }
    Some("OK")
}

#[get("/spotify/id/<show_id>")]
async fn spotify_by_id(
    show_id: String,
    spotify_client: &State<SpotifyClient>,
    redis_client: &State<redis::Client>,
) -> Option<String> {
    SpotifyRss::show_feed(spotify_client, redis_client, show_id)
        .await
        .ok()
}

#[launch]
async fn rocket() -> _ {
    // Config
    let config = settings::Settings::new().unwrap();

    // Logging
    std::env::set_var("RUST_LOG", &config.log_level);
    pretty_env_logger::init();

    // Cache
    let redis_client = crate::cache::client_from_config(&config).unwrap();
    if !redis_client.ping().await {
        panic!("Failed Redis health check");
    }

    info!("Starting Spotify client");
    let spotify_client = spotify::SpotifyClient::from_config(&config).await.unwrap();

    info!("Starting server");
    rocket::build()
        .manage(redis_client)
        .manage(spotify_client)
        .mount("/", routes![index, health, spotify_by_id])
}

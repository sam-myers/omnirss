#[macro_use]
extern crate rocket;
extern crate log;
extern crate pretty_env_logger;
extern crate redis;
#[macro_use]
extern crate serde_derive;

use crate::spotify::*;

mod error;
mod settings;
mod spotify;

use log::info;
use rocket::State;

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

#[get("/spotify/id/<show_id>")]
async fn spotify_by_id(show_id: String, spotify_client: &State<SpotifyClient>, redis_client: &State<redis::Client>) -> Option<String> {
    SpotifyRss::show_feed(spotify_client, redis_client, show_id).await.ok()
}

#[launch]
async fn rocket() -> _ {
    // Config
    let config = settings::Settings::new().unwrap();

    // Logging
    std::env::set_var("RUST_LOG", &config.log_level);
    pretty_env_logger::init();

    info!("Starting Redis client");
    let redis_client = redis::Client::open(config.redis.get_connection_url().unwrap()).unwrap();
    let mut redis_con = redis_client.get_connection().unwrap();
    redis::pipe().cmd("PING").ignore().query::<()>(&mut redis_con).unwrap();

    info!("Starting Spotify client");
    let spotify_client = spotify::SpotifyClient::from_config(&config).await.unwrap();

    info!("Starting server");
    rocket::build()
        .manage(redis_client)
        .manage(spotify_client)
        .mount("/", routes![index, spotify_by_id])
}

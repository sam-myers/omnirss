#[macro_use]
extern crate rocket;
extern crate log;
extern crate pretty_env_logger;
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
async fn spotify_by_id(show_id: String, client: &State<SpotifyClient>) -> Option<String> {
    SpotifyRss::show_feed(client, show_id).await.ok()
}

#[launch]
async fn rocket() -> _ {
    // Config
    let config = settings::Settings::new().unwrap();

    // Logging
    std::env::set_var("RUST_LOG", &config.log_level);
    pretty_env_logger::init();

    info!("Starting Spotify client");
    let spotify_client = spotify::SpotifyClient::from_config(&config).await.unwrap();

    info!("Starting server");
    rocket::build()
        .manage(spotify_client)
        .mount("/", routes![index, spotify_by_id])
}

#[macro_use]
extern crate rocket;
extern crate log;
extern crate pretty_env_logger;

use crate::spotify::*;

mod error;
mod spotify;

use error::Result;
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
    pretty_env_logger::init();

    info!("Starting Spotify client");
    let spotify_client = spotify::SpotifyClient::new().await.unwrap();

    info!("Starting server");
    rocket::build()
        .manage(spotify_client)
        .mount("/", routes![index, spotify_by_id])
}

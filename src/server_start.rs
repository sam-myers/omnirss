use crate::cache::{Cache, RedisCache};
use crate::{routes, settings, spotify};
use log::info;
use rocket_dyn_templates::Template;

#[allow(unused_must_use)]
pub async fn server_start() {
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
        .mount("/", routes::routes())
        .attach(Template::fairing())
        .launch()
        .await;
}

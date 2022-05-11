use crate::cache::{Cache, RedisCache};
use crate::{routes, settings::Settings, spotify};
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use tracing::info;
use tracing_subscriber::prelude::*;

#[allow(unused_must_use)]
pub async fn server_start() {
    // Tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::EnvFilter::from_env("ROCKET_LOG_LEVEL")
                .add_directive("hyper=off".parse().expect("trace directive syntax"))
                .add_directive("reqwest=error".parse().expect("trace directive syntax"))
                .add_directive("rocket=error".parse().expect("trace directive syntax"))
                .add_directive("_=error".parse().expect("trace directive syntax")),
        )
        .init();

    // Config
    let rocket_builder = rocket::build();
    let settings: Settings = rocket_builder.figment().extract().expect("config");

    // Cache
    let redis_cache = RedisCache::from_settings(&settings).expect("redis client");
    if !(&redis_cache).ping().await {
        panic!("Failed Redis health check");
    }

    // Spotify API
    info!("Starting Spotify client");
    let spotify_client = spotify::SpotifyClient::from_settings(&settings)
        .await
        .expect("spotify client");

    info!("Starting server");
    rocket_builder
        .manage(redis_cache)
        .manage(spotify_client)
        .manage(settings)
        .mount("/", FileServer::from("public"))
        .mount("/", routes::routes())
        .attach(Template::fairing())
        .launch()
        .await;
}

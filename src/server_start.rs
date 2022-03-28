use crate::cache::{Cache, RedisCache};
use crate::{routes, settings::Settings, spotify};
use log::info;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

#[allow(unused_must_use)]
pub async fn server_start() {
    // Config
    let rocket_builder = rocket::build();
    let settings: Settings = rocket_builder.figment().extract().expect("config");

    // Sentry
    let _sentry = sentry::init((
        settings.sentry_url.clone(),
        sentry::ClientOptions {
            release: sentry::release_name!(),
            environment: Some(settings.sentry_env.clone().into()),
            sample_rate: settings.sentry_sample_rate,
            ..Default::default()
        },
    ));

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

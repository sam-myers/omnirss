#[tokio::test]
async fn joe_rogan_show_to_rss() {
    let spotify_client = omnirss::spotify::MockSpotifyClient {};
    let cache = omnirss::cache::MemoryCache::new();

    let _feed = omnirss::spotify::SpotifyRss::show_feed(
        spotify_client,
        cache,
        "4rOoJ6Egrf8K2IrywzwOMk".to_string(),
    )
    .await
    .expect("should succeed");
}

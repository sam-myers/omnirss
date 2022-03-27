use std::fs::File;
use std::io::Read;
use std::path::Path;

#[tokio::test]
async fn joe_rogan_show_to_rss() {
    let spotify_client = omnirss::spotify::MockSpotifyClient {};
    let cache: omnirss::cache::MemoryCache = Default::default();

    let feed = omnirss::spotify::SpotifyService::show_feed(
        spotify_client,
        cache,
        "4rOoJ6Egrf8K2IrywzwOMk".to_string(),
    )
    .await
    .expect("should succeed");

    assert_eq!(feed, joe_rogan_show_fixture());
}

fn joe_rogan_show_fixture() -> String {
    let path = Path::new("fixtures/rss/joe_rogan_show.rss");
    let mut file = File::open(&path).expect("fixture missing");
    let mut result = "".to_string();
    if let Err(e) = file.read_to_string(&mut result) {
        panic!("reading file: {}", e);
    }
    result
}

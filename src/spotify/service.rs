use log::debug;
use rss::{ChannelBuilder, Item};

use crate::cache::Cache;
use crate::error::*;
use crate::spotify::cache::{CacheKey, CACHE_SHOW_FOR_SECONDS};
use crate::spotify::Spotify;

pub struct SpotifyService {}

impl SpotifyService {
    // Tries to return the date in RFC 2822 format
    // In: "2021-12-14",
    // Out: "01 Jun 2016 14:31:46 -0700"
    pub fn parse_release_date(date_string: String) -> Result<String> {
        let publish_date_naive =
            chrono::NaiveDate::parse_from_str(date_string.as_str(), "%Y-%m-%d")?;
        Ok(publish_date_naive
            .format("%a, %d %b %Y 15:07:00 -0500")
            .to_string())
    }

    pub async fn show_feed(
        spotify_client: impl Spotify,
        cache: impl Cache,
        show_id: String,
    ) -> Result<String> {
        // Cached show available?
        debug!("Checking cache...");
        let cache_key = CacheKey::show_from_id(&show_id);
        if let Ok(show) = cache.get(&cache_key).await {
            debug!("Using cached feed");
            return Ok(show);
        }

        // Get from API
        debug!("Getting show from Spotify API...");
        let show = spotify_client.get_show(&show_id).await?;

        let title = show.name.clone();
        let link = show.external_urls.spotify.clone();
        let description = show.html_description.clone();

        debug!("Converting Spotify episodes to RSS items");
        let items: Vec<Item> = show
            .episodes
            .items
            .into_iter()
            .map(|i| {
                let pub_date: Option<String> = match i.release_date_precision.as_str() {
                    "day" => SpotifyService::parse_release_date(i.release_date).ok(),
                    _ => None,
                };

                Item {
                    title: Some(i.name),
                    link: Some(i.external_urls.spotify),
                    description: Some(i.description),
                    author: None,
                    categories: vec![],
                    comments: None,
                    enclosure: None,
                    guid: None,
                    pub_date,
                    source: None,
                    content: Some(i.html_description),
                    extensions: Default::default(),
                    itunes_ext: None,
                    dublin_core_ext: None,
                }
            })
            .collect();

        debug!("Building RSS channel");
        let channel = ChannelBuilder::default()
            .title(title)
            .link(link)
            .description(description)
            .items(items)
            .build();

        // Save to Redis
        debug!("Saving to Redis");
        let channel_string: String = channel.to_string();
        if let Err(e) = cache
            .set(&cache_key, &channel_string, CACHE_SHOW_FOR_SECONDS)
            .await
        {
            warn!("Error saving to Redis {:?}", e);
        }

        Ok(channel_string)
    }
}

#[cfg(test)]
mod tests {
    use crate::spotify::SpotifyService;

    #[test]
    fn parse_release_date() {
        let initial = "2021-12-14".to_string();
        let expected = "Tue, 14 Dec 2021 15:07:00 -0500".to_string();

        let parsed = SpotifyService::parse_release_date(initial).unwrap();
        assert_eq!(parsed, expected);
    }
}

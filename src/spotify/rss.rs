use rss::{ChannelBuilder, Item};

use crate::SpotifyClient;
use crate::error::*;

pub struct SpotifyRss {}

impl SpotifyRss {
    pub async fn show_feed(client: &SpotifyClient, show_id: String) -> Result<String> {
        let show = client.get_shows(show_id).await?;

        let title = show.name.clone();
        let link = show.external_urls.spotify.clone();
        let description = show.html_description.clone();

        let mut items: Vec<Item> = show.episodes.items.into_iter().map(|i| {
            Item {
                title: Some(i.name),
                link: Some(i.external_urls.spotify),
                description: Some(i.description),
                author: None,
                categories: vec![],
                comments: None,
                enclosure: None,
                guid: None,
                // chrono::NaiveDate::parse_from_str(i.release_date.as_str(), "%Y-%m-%d").ok().map(|d| d.)
                pub_date: None,
                source: None,
                content: None,
                extensions: Default::default(),
                itunes_ext: None,
                dublin_core_ext: None
            }
        }).collect();

        let channel = ChannelBuilder::default()
            .title(title)
            .link(link)
            .description(description)
            .items(items)
            .build();

        Ok(channel.to_string())
    }
}

// let channel = ChannelBuilder::default()
// .title("Channel Title")
// .link("http://example.com")
// .description("An RSS feed.")
// .build();
// Ok(channel.to_string())

use rss::validation::Validate;
use rss::{ChannelBuilder, Item};

use crate::error::*;
use crate::SpotifyClient;

pub struct SpotifyRss {}

impl SpotifyRss {
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

    pub async fn show_feed(client: &SpotifyClient, show_id: String) -> Result<String> {
        let show = client.get_shows(show_id).await?;

        let title = show.name.clone();
        let link = show.external_urls.spotify.clone();
        let description = show.html_description.clone();

        let items: Vec<Item> = show
            .episodes
            .items
            .into_iter()
            .map(|i| {
                let pub_date: Option<String> = match i.release_date_precision.as_str() {
                    "day" => SpotifyRss::parse_release_date(i.release_date).ok(),
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

        let channel = ChannelBuilder::default()
            .title(title)
            .link(link)
            .description(description)
            .items(items)
            .build();

        channel.validate()?;
        Ok(channel.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::SpotifyRss;

    #[test]
    fn parse_release_date() {
        let initial = "2021-12-14".to_string();
        let expected = "Tue, 14 Dec 2021 15:07:00 -0500".to_string();

        let parsed = SpotifyRss::parse_release_date(initial).unwrap();
        assert_eq!(parsed, expected);
    }
}

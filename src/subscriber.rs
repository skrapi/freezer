use std::{
    fs::{self, File},
    io::BufReader,
};

use ::futures::future::join_all;
use feed_rs::model::Feed;
use serde::{Deserialize, Serialize};

use crate::feeds::Feeds;

pub fn feed_from_file(file: &str) -> Feed {
    let file = File::open(file).expect("Failed to open file");
    feed_rs::parser::parse(BufReader::new(file)).expect("Failed to read channel from file.")
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Subscriber {
    name: String,
    // TODO: Change email to a newtype with validation
    email: String,
    // TODO: Convert to an actual time period
    time_period_hours: u64,
    feeds: Vec<String>,
}

impl Subscriber {
    #[cfg(test)]
    fn new(email: String) -> Self {
        Self {
            email,
            name: "Sylvan".to_owned(),
            // One week
            time_period_hours: 168,
            feeds: vec![],
        }
    }
    pub fn from_config_file(config_file_path: &str) -> Self {
        let file = fs::read_to_string(config_file_path).expect("Failed to open config file");
        let subscriber: Subscriber = toml::from_str(&file).unwrap();

        subscriber
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn add(&mut self, feed: String) {
        self.feeds.push(feed);
    }

    pub fn delete(&mut self, feed: String) {
        self.feeds.retain_mut(|x| x != &feed);
    }

    pub fn list_subscriptions(&self) -> &Vec<String> {
        &self.feeds
    }

    pub fn time_period_days(&self) -> u64 {
        self.time_period_hours.div_ceil(24).max(1)
    }

    pub async fn collect_all_feeds(&self) -> Result<Feeds, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();

        Ok(Feeds::from(
            join_all(self.feeds.iter().map(async |url| {
                feed_rs::parser::parse(
                    client
                        .get(url)
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap()
                        .as_bytes(),
                )
                .unwrap()
            }))
            .await,
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::subscriber::Subscriber;

    #[test]
    fn test_create_subscriber_from_config_file() {
        // Arrange & Act
        let subscriber = Subscriber::from_config_file("tests/config.toml");

        // Assert
        assert_eq!(
            subscriber,
            Subscriber {
                email: "kaladin@archive.com".into(),
                name: "Kaladin".into(),
                time_period_hours: 168,
                feeds: vec!["https://x86.lol/feed.xml".to_string()]
            }
        )
    }
    #[test]
    fn test_add_feed() {
        // Arrange
        let mut subscriber = Subscriber::new("kaladin@archive.com".into());

        let feed = "test_channel.xml".to_owned();
        // Act
        subscriber.add(feed.clone());

        // Assert
        assert_eq!(subscriber.feeds[0], feed)
    }

    #[test]
    fn test_time_period_days() {
        // Arrange
        let mut subscriber = Subscriber::new("kaladin@archive.com".into());
        subscriber.time_period_hours = 168;

        // Act
        let days = subscriber.time_period_days();

        // Assert
        assert_eq!(days, 7)
    }
}

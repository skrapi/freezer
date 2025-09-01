use std::{
    collections::HashMap,
    fs::{self, File},
    io::BufReader,
};

use ::futures::future::join_all;
use feed_rs::model::Feed;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::hash::Hash;

use crate::feeds::Feeds;

pub fn feed_from_file(file: &str) -> Feed {
    let file = File::open(file).expect("Failed to open file");
    feed_rs::parser::parse(BufReader::new(file)).expect("Failed to read channel from file.")
}

/// Custom deserialize function: Vec -> HashMap
fn deserialize_hashset<'de, D, T>(deserializer: D) -> Result<HashMap<T, T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Eq + Hash + Clone,
{
    let vec = Vec::<[T; 2]>::deserialize(deserializer)?;
    Ok(vec
        .into_iter()
        .map(|entry| (entry[0].clone(), entry[1].clone()))
        .collect())
}

/// Custom serialize function: HashMap -> Vec
fn serialize_hashset<S, T>(hashmap: &HashMap<T, T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize + Clone,
{
    let mut vec: Vec<[T; 2]> = hashmap
        .iter()
        .map(|(key, value)| [key.clone(), value.clone()])
        .collect();
    vec.sort_by(|a, b| {
        // For consistent ordering, we serialize to string and compare
        let a_str = toml::to_string(&a[0]).unwrap_or_default();
        let b_str = toml::to_string(&b[0]).unwrap_or_default();
        a_str.cmp(&b_str)
    });
    vec.serialize(serializer)
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Subscriber {
    name: String,
    // TODO: Change email to a newtype with validation
    email: String,
    // TODO: Convert to an actual time period
    time_period_hours: u64,
    #[serde(
        deserialize_with = "deserialize_hashset",
        serialize_with = "serialize_hashset"
    )]
    feeds: HashMap<String, String>,
}

impl Subscriber {
    #[cfg(test)]
    fn new(email: String) -> Self {
        Self {
            email,
            name: "Sylvan".to_owned(),
            // One week
            time_period_hours: 168,
            feeds: HashMap::new(),
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
    pub fn add(&mut self, feed: String, date: String) -> bool {
        self.feeds.insert(feed, date).is_some()
    }

    pub fn delete(&mut self, feed: &str) -> bool {
        self.feeds.remove(feed).is_some()
    }

    pub fn list_subscriptions(&self) -> &HashMap<String, String> {
        &self.feeds
    }

    pub fn time_period_days(&self) -> u64 {
        self.time_period_hours.div_ceil(24).max(1)
    }

    pub async fn collect_all_feeds(&self) -> Result<Feeds, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();

        Ok(Feeds::from(
            join_all(self.feeds.iter().map(async |(url, _date)| {
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
    use std::collections::HashMap;

    use crate::subscriber::Subscriber;

    #[test]
    fn test_create_subscriber_from_config_file() {
        // Arrange & Act
        let subscriber = Subscriber::from_config_file("tests/config.toml");

        let mut feeds = HashMap::new();
        feeds.insert(
            "https://x86.lol/feed.xml".to_string(),
            "2025-08-21".to_owned(),
        );

        let correct_subscriber = Subscriber {
            email: "kaladin@archive.com".into(),
            name: "Kaladin".into(),
            time_period_hours: 168,
            feeds,
        };

        // Assert
        assert_eq!(subscriber, correct_subscriber)
    }
    #[test]
    fn test_add_feed() {
        // Arrange
        let mut subscriber = Subscriber::new("kaladin@archive.com".into());

        let feed = "test_channel.xml".to_owned();
        // Act
        subscriber.add(feed.clone(), "2025-08-21".to_owned());

        // Assert
        assert_eq!(subscriber.feeds.contains_key(&feed), true)
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

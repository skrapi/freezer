use std::{collections::HashMap, fs::File, io::BufReader};

use rss::{Channel, Item, validation::Validate};

pub fn channel_from_file(file: &str) -> Channel {
    let file = File::open(file).expect("Failed to open file");
    let channel =
        Channel::read_from(BufReader::new(file)).expect("Failed to read channel from file.");

    channel.validate().expect("Invalid RSS file.");
    channel
}

#[derive(Debug, PartialEq)]
pub struct Subscriber {
    name: Option<String>,
    // TODO: Change email to a newtype with validation
    email: String,
    channels: HashMap<String, Channel>,
    // TODO: Convert to an actual time period
    time_period_hours: usize,
}

impl Subscriber {
    fn new(email: String) -> Self {
        Self {
            email,
            name: None,
            channels: HashMap::new(),
            // One week
            time_period_hours: 168,
        }
    }
    pub fn from_config_file(config_file_path: &str) -> Self {
        let file = File::open(config_file_path).expect("Failed to open config file");
        let reader = BufReader::new(file);
        // TODO: Use a serde toml deserialiser or equivalent

        todo!()
    }

    pub fn add(&mut self, channel: Channel) {
        // TODO: use the returned item, maybe as a warning?
        let _ = self.channels.insert(channel.link.clone(), channel);
    }

    pub fn delete(&mut self, channel: Channel) {
        // TODO: use the returned item, maybe as a warning
        let _ = self.channels.remove(&channel.link);
    }

    pub fn send_new_items_in_time_period(&self) {
        todo!()
    }

    fn collect_all_items_in_time_period(&self) -> Vec<Item> {
        todo!()
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_create_subscriber_from_config_file() {
        // Arrange & Act
        let subscriber = Subscriber::from_config_file("test/config.toml");

        // Assert
        assert_eq!(
            subscriber,
            Subscriber {
                email: "kaladin@archive.com".into(),
                name: Some("Kaladin".into()),
                channels: HashMap::new(),
                time_period_hours: 168
            }
        )
    }
    #[test]
    fn test_add_channel() {
        // Arrange
        let mut subscriber = Subscriber::new("kaladin@archive.com".into());
        let channel = channel_from_file("tests/feed.xml");

        // Act
        subscriber.add(channel.clone());

        // Assert
        assert_eq!(
            subscriber.channels.into_keys().next().unwrap(),
            channel.link
        )
    }

    #[test]
    fn test_get_all_items_in_time_period() {
        // Arrange
        let mut subscriber = Subscriber::new("kaladin@archive.com".into());
        let channel = channel_from_file("tests/feed.xml");
        subscriber.add(channel.clone());
        let actual_items = channel.into_items().split_off(1);

        // TODO: Figure out how to declare a static date;
        // needs to be less than a week after
        // Sun, 10 Aug 2025 00:00:00 +0000
        let current_date = todo!();

        // Act
        let collected_items = subscriber.collect_all_items_in_time_period();

        // Assert
        assert_eq!(collected_items, actual_items);
    }
}

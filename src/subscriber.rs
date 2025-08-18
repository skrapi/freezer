use std::{collections::HashMap, fs::File, io::BufReader};

use rss::{Channel, validation::Validate};

pub fn channel_from_file(file: &str) -> Channel {
    let file = File::open(file).expect("Failed to open file");
    let channel =
        Channel::read_from(BufReader::new(file)).expect("Failed to read channel from file.");

    channel.validate().expect("Invalid RSS file.");
    channel
}
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
}

mod tests {
    use super::*;
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
}

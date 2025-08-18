use rss::Channel;
use rss::validation::Validate;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

pub fn channel_from_file(file: &str) -> Channel {
    let file = File::open(file).expect("Failed to open file");
    let channel =
        Channel::read_from(BufReader::new(file)).expect("Failed to read channel from file.");

    channel.validate().expect("Invalid RSS file.");
    channel
}

struct Subscriber {
    name: Option<String>,
    // TODO: Change email to a newtype with validation
    email: String,
    channels: HashMap<String, Channel>,
    // TODO: Convert to an actual time period
    time_period_hours: usize,
}

impl Subscriber {
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

/// TODO: Add clap arguments for the following
/// Actions
/// subscribe
/// - email to subscribe (optional with config)
/// - feed to subscribe to
/// unsubscribe
/// - email to unsubscribe (optional with config)
/// - feed to unsubscribe
/// send_email
/// - email to send email to

fn main() {
    let input_file_str = "feed.xml";

    let feed = channel_from_file(input_file_str);

    println!(
        "We have a channel with the following title: {}",
        feed.title()
    );
}

// Plan 1: Read in a RSS xml file and generate an html file to read
use rss::Channel;
use rss::validation::Validate;
use std::fs::File;
use std::io::BufReader;

struct Feed {
    channel: Channel,
}

impl Feed {
    pub fn from_file(file: &str) -> Self {
        let file = File::open(file).expect("Failed to open file");
        let channel =
            Channel::read_from(BufReader::new(file)).expect("Failed to read channel from file.");

        channel.validate().expect("Invalid RSS file.");
        Self { channel }
    }
    pub fn from_url(url: &str) -> Self {
        todo!()
    }

    pub fn title(&self) -> &str {
        &self.channel.title
    }
}

fn main() {
    let input_file_str = "feed.xml";

    let feed = Feed::from_file(input_file_str);

    println!(
        "We have a channel with the following title: {}",
        feed.title()
    );
}

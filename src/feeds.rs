use std::{fs::File, io::BufReader};

use chrono::{DateTime, Utc};
use feed_rs::model::{Entry, Feed};

pub struct Feeds {
    feeds: Vec<Feed>,
}

impl Feeds {
    pub fn new() -> Self {
        Self { feeds: Vec::new() }
    }

    pub fn add_from_file(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(file)?;
        self.feeds
            .push(feed_rs::parser::parse(BufReader::new(file))?);
        Ok(())
    }
    pub fn get_new_entries(&self, since: DateTime<Utc>) -> Vec<Entry> {
        Vec::new()
    }
}

mod tests {
    use chrono::{DateTime, TimeZone, Utc};

    use crate::feeds::Feeds;

    #[test]
    fn test_get_new_entries() {
        // Arrange
        let now: DateTime<Utc> = Utc.with_ymd_and_hms(2025, 08, 14, 12, 30, 00).unwrap();
        let mut feeds = Feeds::new();
        feeds.add_from_file("tests/rss.xml");
        feeds.add_from_file("tests/atom.xml");

        // Act
        let retrieved_entries = feeds.get_new_entries(now);
        println!("{retrieved_entries:?}");

        // Assert
    }
}

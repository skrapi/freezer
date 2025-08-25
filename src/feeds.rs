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
    pub fn get_new_entries(&self, since: DateTime<Utc>) -> Vec<&Entry> {
        self.feeds
            .iter()
            .flat_map(|feed| {
                feed.entries
                    .iter()
                    .filter(|entry| entry.published.map_or(false, |date| date > since))
            })
            .collect()
    }
}

#[derive(Debug, PartialEq)]
pub struct SimpleEntry {
    title: String,
    link: String,
}

impl SimpleEntry {
    pub fn new(title: String, link: String) -> Self {
        Self {
            title: title,
            link: link,
        }
    }
    pub fn from_entry(entry: &Entry) -> Self {
        let title = entry.title.clone().unwrap().content.clone();
        let link = entry.links.first().unwrap().href.clone();

        Self { title, link }
    }
}
mod tests {
    use chrono::{DateTime, TimeZone, Utc};
    use feed_rs::model::Entry;

    use crate::feeds::{Feeds, SimpleEntry};

    #[test]
    fn test_get_new_entries() {
        // Arrange
        let now: DateTime<Utc> = Utc.with_ymd_and_hms(2025, 07, 31, 12, 30, 00).unwrap();
        let mut feeds = Feeds::new();
        feeds.add_from_file("tests/rss.xml").unwrap();
        feeds.add_from_file("tests/atom.xml").unwrap();

        // Act
        let retrieved_entries = feeds.get_new_entries(now);

        // Assert
        // TODO: Create a better test result, validate that the entries are parsed correctly.
        assert_eq!(retrieved_entries.len(), 2);
    }

    #[test]
    fn test_get_simple_entries() {
        // Arrange
        let now: DateTime<Utc> = Utc.with_ymd_and_hms(2025, 07, 31, 12, 30, 00).unwrap();
        let mut feeds = Feeds::new();
        feeds.add_from_file("tests/rss.xml").unwrap();
        feeds.add_from_file("tests/atom.xml").unwrap();

        // Act
        let retrieved_entries: Vec<SimpleEntry> = feeds
            .get_new_entries(now)
            .iter()
            .map(|entry| SimpleEntry::from_entry(entry))
            .collect();

        // Assert
        let expected_simple_entries = vec![
            SimpleEntry::new(
                "Quick and Dirty Website Change Monitoring".to_owned(),
                "https://x86.lol/generic/2025/08/10/change-monitoring.html".to_owned(),
            ),
            SimpleEntry::new(
                "Code Review Can Be Better".to_owned(),
                "https://tigerbeetle.com/blog/2025-08-04-code-review-can-be-better".to_owned(),
            ),
        ];

        assert_eq!(retrieved_entries, expected_simple_entries);
    }
}

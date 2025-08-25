use chrono::{DateTime, Utc};
use feed_rs::model::{Entry, Feed};

pub struct Feeds {
    feeds: Vec<Feed>,
}

impl Feeds {
    pub fn new() -> Self {
        Self { feeds: Vec::new() }
    }

    pub fn get_new_entries(&self, since: DateTime<Utc>) -> Vec<Entry> {
        todo!()
    }
}

mod tests {
    #[test]
    fn test_get_new_entries() {
        // Arrange
        // Act
        // Assert
    }
}

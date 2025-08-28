use chrono::{DateTime, Utc};

use crate::feeds::SimpleEntry;

pub struct Digest {
    entries: Vec<SimpleEntry>,
    publish_date: DateTime<Utc>,
    recipient_name: String,
}

impl Digest {
    pub fn new(
        entries: Vec<SimpleEntry>,
        publish_date: DateTime<Utc>,
        recipient_name: String,
    ) -> Self {
        Self {
            entries,
            publish_date,
            recipient_name,
        }
    }

    pub fn plaintext(&self) -> String {
        let mut string_list = vec![
            format!(
                "Hello {},\n\nHere are new articles from subcribed feeds, as of {}.",
                self.recipient_name,
                self.publish_date.to_rfc3339().split_once("T").unwrap().0
            )
            .to_owned(),
        ];
        let mut article_list = self
            .entries
            .iter()
            .map(|entry| {
                format!("{} - {}\n{}", entry.title, entry.publish_date, entry.link).to_owned()
            })
            .collect::<Vec<String>>();
        let sign_off = "Kind regards,\nSylvan".to_owned();

        string_list.append(&mut article_list);
        string_list.push(sign_off);
        string_list.join("\n\n")
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, TimeZone, Utc};

    use crate::{digest::Digest, feeds::SimpleEntry};

    #[test]
    fn test_plaintext() {
        // Arrange
        let entries = vec![
            SimpleEntry {
                title: "Quick and Dirty Website Change Monitoring".to_owned(),
                link: "https://x86.lol/generic/2025/08/10/change-monitoring.html".to_owned(),
                publish_date: "2025-08-10".to_owned(),
            },
            SimpleEntry {
                title: "Code Review Can Be Better".to_owned(),
                link: "https://tigerbeetle.com/blog/2025-08-04-code-review-can-be-better"
                    .to_owned(),
                publish_date: "2025-08-04".to_owned(),
            },
        ];

        let now: DateTime<Utc> = Utc.with_ymd_and_hms(2025, 08, 11, 12, 30, 00).unwrap();
        let digest = Digest::new(entries, now, "Sylvan".to_owned());

        let expected_plaintext = r"Hello Sylvan,

Here are new articles from subcribed feeds, as of 2025-08-11.

Quick and Dirty Website Change Monitoring - 2025-08-10
https://x86.lol/generic/2025/08/10/change-monitoring.html

Code Review Can Be Better - 2025-08-04
https://tigerbeetle.com/blog/2025-08-04-code-review-can-be-better

Kind regards,
Sylvan";

        // Act
        let plaintext = digest.plaintext();

        // Assert
        assert_eq!(plaintext, expected_plaintext);
    }
}

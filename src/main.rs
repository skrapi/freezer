use std::fmt::format;

use chrono::{Date, DateTime, Days, Local, Utc};
use clap::{Parser, Subcommand};
use feed_rs::model::{Feed, Text};
use freezer::configuration::Configuration;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Command to perform
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Subscribe to new feed
    Add {
        /// Feed to subscribe to
        #[arg(short, long)]
        feed: String,
    },
    /// Unsubscribe from feed
    Remove {
        /// Feed to unsubscribe from
        #[arg(short, long)]
        feed: String,
    },
    /// List all subscriptions
    List,
    /// Publish a collection of the latest feed contents
    Publish {
        /// Email address of subscriber
        #[arg(short, long)]
        email: Option<String>,
    },
}

use home::home_dir;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub async fn send_digest(
    to_name: String,
    to_email: String,
    body: String,
    cred_email: String,
    cred_password: String,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create email message
    let email = Message::builder()
        .from("Freezer <freezer@sylvansmit.com>".parse()?)
        .to(format!("{to_name} <{to_email}>").parse()?)
        .subject(format!(
            "Freezer Digest - {}",
            chrono::Local::now().format("%Y-%m-%d")
        ))
        .header(ContentType::TEXT_PLAIN)
        .body(body)?;

    let creds = Credentials::new(cred_email, cred_password);

    // Create SMTP transport
    let mailer = SmtpTransport::relay("smtp.fastmail.com")?
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => println!("Could not send email: {e:?}"),
    }

    Ok(())
}

fn parse_feed(feed: &Feed, date: DateTime<Utc>) -> Option<String> {
    // Find all items/entries that have been created after date
    // Create a url, image and title combindation and pass to string to send
    // Return None if no valid posts

    let entries = feed
        .entries
        .iter()
        .filter(|entry| {
            if let Some(published) = entry.published
                && published > date
            {
                true
            } else {
                false
            }
        })
        .map(|entry| format!("Title: {:?}, url: {:?}", entry.title, entry.links))
        .collect::<Vec<String>>();

    if entries.is_empty() {
        None
    } else {
        Some(entries.join("\n"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_file_path = ".config/freezer/freezer.toml";
    let (mut config, config_file_path) = if let Some(path) = home_dir()
        && path.exists()
        && path.join(config_file_path).exists()
    {
        (
            Configuration::from_config_file(path.join(config_file_path)),
            path.join(config_file_path),
        )
    } else {
        println!("You appear to be missing your config file");
        return Ok(());
    };

    let cli = Cli::parse();
    match cli.command {
        Commands::Add { feed } => {
            config.subscriber.add(feed.clone());
            config.save(config_file_path);
            println!("Added {feed} to subscriptions.");
        }
        Commands::Remove { feed } => {
            config.subscriber.delete(feed.clone());
            config.save(config_file_path);
            println!("Removed {feed} from subscriptions.");
        }
        Commands::List => println!("{:?}", config.subscriber.list_subscriptions()),
        Commands::Publish { email } => {
            let feeds = format!(
                "{:?}",
                config
                    .subscriber
                    .collect_all_feeds()
                    .await?
                    .iter()
                    .map(|feed| parse_feed(
                        feed,
                        Utc::now().checked_sub_days(Days::new(7)).unwrap()
                    ))
            );
            send_digest(
                "Sylvan".to_owned(),
                email.unwrap(),
                feeds,
                config.sender.app_email,
                config.sender.app_password,
            )
            .await?
        }
    };
    Ok(())
}

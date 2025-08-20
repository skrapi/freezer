use clap::{Parser, Subcommand};
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
        /// Email address of subscriber
        #[arg(short, long)]
        email: Option<String>,

        /// Feed to subscribe to
        #[arg(short, long)]
        feed: String,
    },
    /// Unsubscribe from feed
    Remove {
        /// Email address of subscriber
        #[arg(short, long)]
        email: Option<String>,

        /// Feed to unsubscribe from
        #[arg(short, long)]
        feed: String,
    },
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

pub async fn send_wrap_up_email(
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
        .subject("Test email from Rust!")
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = if let Some(path) = home_dir()
        && path.exists()
        && path.join(".config/freezer/freezer.toml").exists()
    {
        Configuration::from_config_file(path.join(".config/freezer/freezer.toml"))
    } else {
        println!("You appear to be missing your config file");
        return Ok(());
    };

    let cli = Cli::parse();
    match cli.command {
        Commands::Add { email, feed } => todo!(),
        Commands::Remove { email, feed } => todo!(),
        Commands::Publish { email } => {
            send_wrap_up_email(
                "Sylvan".to_owned(),
                email.unwrap(),
                "Test publish command".to_owned(),
                config.sender.app_email,
                config.sender.app_password,
            )
            .await?
        }
    };
    Ok(())
}

use clap::{Parser, Subcommand};
use freezer::subscriber::Subscriber;

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

fn main() {
    let subscriber = Subscriber::from_config_file("/home/skrapi/.config/freezer/freezer.toml");

    println!("{subscriber:?}");

    let cli = Cli::parse();

    println!("{cli:?}");
}

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
    Add {
        #[arg(short, long)]
        email: Option<String>,

        #[arg(short, long)]
        feed: String,
    },
    Remove {
        #[arg(short, long)]
        email: Option<String>,

        #[arg(short, long)]
        feed: String,
    },
    Publish {
        #[arg(short, long)]
        email: Option<String>,
    },
}

fn main() {
    // let subscriber = Subscriber::from_config_file("/home/skrapi/.config/freezer/freezer.toml");

    let cli = Cli::parse();

    println!("{cli:?}");
}

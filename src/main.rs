use freezer::subscriber::Subscriber;

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
    let subscriber = Subscriber::from_config_file("/home/skrapi/.config/freezer/freezer.toml");
}

use std::{fs, path::PathBuf};

use crate::subscriber::Subscriber;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Sender {
    pub app_password: String,
    pub app_email: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Configuration {
    pub subscriber: Subscriber,
    pub sender: Sender,
}

impl Configuration {
    pub fn from_config_file(config_file_path: PathBuf) -> Self {
        let file = fs::read_to_string(config_file_path).expect("Failed to open config file");
        toml::from_str(&file).unwrap()
    }
}

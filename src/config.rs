use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::process::exit;

use serde::{Deserialize, Serialize};

use crate::messages::{push_message, Type};

#[derive(Serialize, Deserialize)]
pub(crate) struct Config {
    pub github_username: String,
    pub blog_title: String,
    pub blog_description: String,
    pub blog_url: String,
    pub pages_title: String,
    pub minify_html: bool,
    pub show_comments: bool,
}

static CONFIG_FILE: &str = "gisture.json";

impl Config {
    pub fn default() -> Self {
        // default config file values
        Self {
            github_username: "octocat".into(),
            blog_title: "John Wick's Pencil".into(),
            blog_description: "Rusty, I Guess.".into(),
            blog_url: "https://blog.johnwickspencil.com/".into(),
            pages_title: "{{ blog_title }} | John Wick's Pencil".into(),
            minify_html: false,
            show_comments: true,
        }
    }

    /// Generate a boilerplate config file
    pub fn generate_default() {
        if !Path::new(CONFIG_FILE).exists() {
            let json_config = match serde_json::to_string_pretty(&Self::default()) {
                Ok(json_config) => json_config,
                Err(error) => {
                    let message = format!("Failed to serialize JSON config: \n\t{}", error);
                    push_message(Type::Error, &message);
                    exit(1)
                }
            };
            let mut config_file = match File::create(CONFIG_FILE) {
                Ok(file) => file,
                Err(error) => {
                    let message = format!("Failed to create config file: \n\t{}", error);
                    push_message(Type::Error, &message);
                    exit(1)
                }
            };
            match config_file.write_all(json_config.as_bytes()) {
                Ok(()) => (),
                Err(error) => {
                    let message = format!("Failed to write to config file: \n\t{}", error);
                    push_message(Type::Error, &message);
                    exit(1)
                }
            }
        }
    }

    /// Fetch config values from `gisture.json`
    pub fn get_config() -> Self {
        let file = match File::open(CONFIG_FILE) {
            Ok(file) => file,
            Err(error) => {
                let message = format!("Failed to open config file: \n\t{}", error);
                push_message(Type::Error, &message);
                exit(1)
            }
        };

        let mut buf_reader = BufReader::new(file);
        let mut user_config = String::new();
        match buf_reader.read_to_string(&mut user_config) {
            Ok(_) => (),
            Err(error) => {
                let message = format!("Failed while reading config file: \n\t{}", error);
                push_message(Type::Error, &message);
                exit(1)
            }
        }

        let config: Config = match serde_json::from_str(&user_config) {
            Ok(config) => config,
            Err(error) => {
                let message = format!("Failed while serializing user config: \n\t{}", error);
                push_message(Type::Error, &message);
                exit(1)
            }
        };

        config
    }
}

use std::{fs::File, path::PathBuf};

use chrono::{Local, NaiveDate};
use directories::BaseDirs;
use ron::{de::from_reader, ser::to_writer};
use serde::{Deserialize, Serialize};

pub fn today() -> NaiveDate {
    Local::now().date_naive()
}

const CONFIG_DIR_NAME: &str = "mood";
pub const CONFIG_NAME: &str = "config.ron";
pub const DEFAULT_JOURNAL_NAME: &str = "journal.ron";

#[derive(Serialize, Deserialize)]
pub struct MoodConfig {
    pub journal_dir: PathBuf,
}
impl Default for MoodConfig {
    fn default() -> Self {
        let config_dir = get_config_dir();
        let config_file_path = get_config_file_path();
        let file = match File::open(&config_file_path) {
            Ok(file) => file,
            Err(_) => {
                println!("No existing config file found, creating one...");
                match std::fs::create_dir(config_dir) {
                    Ok(_) => (),
                    Err(e) => match e.kind() {
                        std::io::ErrorKind::AlreadyExists => (),
                        _ => panic!("Error occured when creating config folder: {e}"),
                    },
                }
                File::create(config_file_path).expect("able to create config file.")
            }
        };

        match from_reader(&file) {
            Ok(config) => config,
            Err(_) => {
                let journal_dir = get_default_journal_dir().join(DEFAULT_JOURNAL_NAME);
                let config = MoodConfig { journal_dir };

                to_writer(&file, &config).expect("save default config to file.");
                println!("Created default config.ron.");
                config
            }
        }
    }
}

pub fn get_config_dir() -> PathBuf {
    let base_dir = BaseDirs::new().expect("able to get base dirs for platform");
    base_dir.config_local_dir().join(CONFIG_DIR_NAME)
}

pub fn get_config_file_path() -> PathBuf {
    get_config_dir().join(CONFIG_NAME)
}

pub fn get_default_journal_dir() -> PathBuf {
    get_config_dir()
}

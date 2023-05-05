use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
};

use directories::BaseDirs;
use ron::{de::from_reader, ser::to_writer};
use serde::{Deserialize, Serialize};

use crate::{error::MoodError, journal::DEFAULT_JOURNAL_NAME};

const CONFIG_DIR_NAME: &str = "mood";
pub const CONFIG_NAME: &str = "config.ron";

#[derive(Serialize, Deserialize, Debug)]
pub struct MoodConfig {
    pub journal_path: PathBuf,
}

impl Default for MoodConfig {
    fn default() -> Self {
        Self {
            journal_path: get_config_dir()
                .expect("mood has created the configuration directory by this point")
                .join(DEFAULT_JOURNAL_NAME),
        }
    }
}

impl MoodConfig {
    pub fn init() -> Result<Self, Box<dyn std::error::Error>> {
        let dir = match get_config_dir() {
            Some(dir) => dir,
            None => create_config_dir()?,
        };

        let config_file_path = dir.join(CONFIG_NAME);

        let config = match config_file_path.is_file() {
            true => {
                let file = File::open(config_file_path)?;
                let config: MoodConfig = from_reader(file)?;
                config
            }
            false => {
                let config = Self::default();
                config.save_to_file()?;
                config
            }
        };
        Ok(config)
    }

    pub fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let dir = match get_config_dir() {
            Some(dir) => dir,
            None => create_config_dir()?,
        };

        let config_file_path = dir.join(CONFIG_NAME);
        let config_file = match config_file_path.exists() {
            true => OpenOptions::new().write(true).open(config_file_path)?,
            false => File::create(config_file_path)?,
        };

        to_writer(config_file, self)
            .map_err(|e| MoodError::ConfigFileError(format!("Write to file. I/O error: {e:?}")))?;
        Ok(())
    }
}

fn get_config_dir() -> Option<PathBuf> {
    let base_dir = BaseDirs::new().expect("able to get base dirs for platform");
    let config_dir = base_dir.config_local_dir().join(CONFIG_DIR_NAME);
    match config_dir.exists() {
        true => Some(config_dir),
        false => None,
    }
}

fn create_config_dir() -> Result<PathBuf, std::io::Error> {
    let base_dir = BaseDirs::new().expect("able to get base dirs for platform");
    let config_dir = base_dir.config_local_dir().join(CONFIG_DIR_NAME);
    println!("[INFO] Creating config dir");
    std::fs::create_dir(&config_dir)?;
    Ok(config_dir)
}

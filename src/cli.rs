use std::{
    fs::{File},
    path::PathBuf,
    str::FromStr,
};

use chrono::{Days, NaiveDate};
use clap::Subcommand;
use mood::{get_config_file_path, today, MoodConfig};
use ron::ser::to_writer;

use crate::{
    journal::{save_journal, Journal, JournalEntry},
    rating::Rating,
};
use clap::Parser;

#[derive(Parser)]
#[command(author, version, arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add entry to journal.
    Add {
        #[clap(value_enum)]
        rating: Rating,
        #[clap(value_parser)]
        note: Option<String>,
    },
    /// Get a specific entry by date.
    Get {
        /// The date to get
        #[clap(value_parser)]
        date: Option<NaiveDate>,
    },
    /// List journal entries
    List {
        /// The date to check from <YYYY-MM-DD>
        #[clap(value_parser)]
        from: Option<NaiveDate>,
        /// The date to check to <YYYY-MM-DD>
        #[clap(value_parser)]
        to: Option<NaiveDate>,
    },
    /// Set program options
    Config {
        /// The absolute path to the file (not just directory) that stores the journal data. If it does not exist, mood will create it.
        #[arg(short)]
        file: String,
    },
}

pub fn add(config: &MoodConfig, journal: &mut Journal, mood: &Rating, note: &Option<String>) {
    let entry = JournalEntry::new(today(), mood.clone(), note.to_owned().unwrap_or_default());
    journal.add_entry(entry);
    save_journal(config, journal).expect("save file should always succeed");
}

pub fn get(journal: &Journal, date: &Option<NaiveDate>) {
    match date {
        Some(date) => match journal.get(date) {
            Some(entry) => {
                println!("[{}]\t{:?}\t{}", entry.date, entry.mood, entry.note)
            }
            None => println!("No entry found."),
        },
        None => match journal.get(&today()) {
            Some(entry) => {
                println!("[{}]\t{:?}\t{}", entry.date, entry.mood, entry.note)
            }
            None => println!("No entry found."),
        },
    }
}

pub fn list(journal: &Journal, from: &Option<NaiveDate>, to: &Option<NaiveDate>) {
    let (from, to) = match (from, to) {
        (None, None) => (
            today()
                .checked_sub_days(Days::new(14))
                .expect("two weeks back in time is fine"),
            today(),
        ),
        (None, Some(to)) => (
            today()
                .checked_sub_days(Days::new(14))
                .expect("two weeks back in time is fine"),
            *to,
        ),
        (Some(from), None) => (*from, today()),
        (Some(from), Some(to)) => (*from, *to),
    };

    println!();
    println!("Listing entries from {from:?} to {to:?}");
    println!("---------------------------------------");
    for entry in journal.get_entries(&from, &to).expect("valid date range") {
        println!("[{}]\t{:?}\t{}", entry.date, entry.mood, entry.note)
    }
    println!();
}

pub fn config(config: &mut MoodConfig, file: &str) {
    let file = if let Ok(file) = PathBuf::from_str(&file.replace('\n', "")) {
        file
    } else {
        println!(
            "Expected a valid journal directory path. Keeping old path {:?}",
            config.journal_dir
        );
        return;
    };

    config.journal_dir = file;

    let file =
        File::create(get_config_file_path()).expect("config file created at first run of cli");
    to_writer(file, &config).expect("able towrite to config file");
}

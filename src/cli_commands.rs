use std::{path::PathBuf, str::FromStr};

use chrono::{Days, NaiveDate};
use clap::Parser;
use clap::Subcommand;

use crate::config::MoodConfig;
use crate::helpers::today;
use crate::journal::Journal;
use crate::journal::JournalEntry;
use crate::rating::Rating;

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
        path_to_journal: String,
    },
}

pub fn add(config: &MoodConfig, journal: &mut Journal, mood: &Rating, note: &Option<String>) {
    let entry = JournalEntry::new(today(), mood.clone(), note.to_owned().unwrap_or_default());
    journal.add_entry(entry);
    journal
        .save(config)
        .expect("save file should always succeed");
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
            config.journal_path
        );
        return;
    };

    config.journal_path = file;
    config
        .save_to_file()
        .expect("the updated configuration is saved to file")
}

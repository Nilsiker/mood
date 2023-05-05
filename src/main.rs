mod cli_commands;
mod config;
mod error;
mod helpers;
mod journal;
mod rating;

use clap::Parser;
use cli_commands::{add, config, get, list, Cli, Commands};
use config::MoodConfig;
use journal::Journal;

fn main() {
    let mut mood_config = MoodConfig::init().expect("configuration is initialized without issues");
    let cli = Cli::parse();
    let mut journal = Journal::init(&mood_config).unwrap_or_default();
    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(command) => match command {
            Commands::Add { rating: mood, note } => add(&mood_config, &mut journal, mood, note),
            Commands::List { from, to } => list(&journal, from, to),
            Commands::Get { date } => get(&journal, date),
            Commands::Config { path_to_journal: file } => config(&mut mood_config, file),
        },
        None => (),
    }
}

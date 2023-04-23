mod cli;
mod journal;
mod rating;

use clap::Parser;
use cli::{add, config, get, list, Cli, Commands};
use mood::MoodConfig;

fn main() {
    let mut mood_config = MoodConfig::default();
    let cli = Cli::parse();
    let mut journal = journal::load_journal(&mood_config).unwrap_or_default();
    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(command) => match command {
            Commands::Add { rating: mood, note } => add(&mood_config, &mut journal, mood, note),
            Commands::List { from, to } => list(&journal, from, to),
            Commands::Get { date } => get(&journal, date),
            Commands::Config { file } => config(&mut mood_config, file),
        },
        None => (),
    }
}

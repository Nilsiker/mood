mod cli;
mod journal;
mod rating;

use clap::Parser;
use cli::{add, list, Cli, Commands, get};

fn main() {
    let cli = Cli::parse();
    let mut journal = journal::load_journal().unwrap_or_default();
    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(command) => match command {
            Commands::Add { rating: mood, note } => add(&mut journal, mood, note),
            Commands::List { from, to } => list(&journal, from, to),
            Commands::Get { date } => get(&journal,date)
        },
        None => (),
    }
}

use clap::Parser;

mod cli;
mod commands;
mod common;
mod storage;
mod structs;

fn main() {
    let cli = cli::Cli::parse();

    match &cli.command {
        cli::Commands::Update => commands::update(),
        cli::Commands::Search { search_term, max } => {
            commands::search(search_term, max)
        }
        cli::Commands::Track { show } => {
            commands::track(show)
        }
        cli::Commands::Untrack { show } => {
            commands::untrack(show)
        }
        cli::Commands::List => commands::list(),
        cli::Commands::Check => commands::check(),
    }
}

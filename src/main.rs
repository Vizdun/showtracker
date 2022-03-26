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
        cli::Commands::Track { id } => commands::track(id),
        cli::Commands::Untrack { id } => {
            commands::untrack(id)
        }
        cli::Commands::List => commands::list(),
        cli::Commands::Check => commands::check(),
    }
}

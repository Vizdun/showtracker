use clap::Parser;
use cli::Commands;

mod cli;
mod commands;
mod common;
mod storage;
mod structs;

fn main() {
    let cli = cli::Cli::parse();

    match &cli.command {
        #[cfg(debug_assertions)]
        Commands::Debug => {
            commands::debug();
        }
        Commands::Update { show, full } => {
            commands::update(show.clone(), *full)
        }
        Commands::Search {
            search_term,
            max,
            id,
            stars,
            numeral,
        } => commands::search(
            search_term,
            max,
            *id,
            *stars,
            numeral.clone(),
        ),
        Commands::Track { show } => commands::track(show),
        Commands::Untrack { show } => {
            commands::untrack(show)
        }
        Commands::List { id, tree, numeral } => {
            commands::list(*id, *tree, numeral.clone())
        }
        Commands::Check { keep } => commands::check(*keep),
    }
}

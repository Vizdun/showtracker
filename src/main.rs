use clap::{AppSettings, Parser, Subcommand};

mod commands;
mod common;
mod storage;
mod structs;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(
    AppSettings::UseLongFormatForHelpSubcommand
))]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Checks if there are any new episodes
    // istg, if this makes descriptions
    Check,
    /// Lists tracked shows
    List,
    /// Searches the show list
    Search {
        search_term: String,
        #[clap(default_value_t = 5)]
        max: u32,
    },
    /// Starts tracking a show
    Track {
        id: String,
    },
    /// Stops tracking a show
    Untrack {
        id: String,
    },
    /// Updates the show list
    Update,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Update => commands::update::main(),
        Commands::Search { search_term, max } => {
            commands::search::main(search_term, max)
        }
        Commands::Track { id } => commands::track::main(id),
        Commands::Untrack { id } => {
            commands::untrack::main(id)
        }
        Commands::List => commands::list::main(),
        Commands::Check => commands::check::main(),
    }
}

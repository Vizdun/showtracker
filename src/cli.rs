use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Checks if there are any new episodes
    Check,
    /// Lists tracked shows
    List,
    /// Searches the show list
    Search {
        search_term: String,
        #[clap(default_value_t = 5)]
        max: u32,
        #[clap(short, long)]
        regex: bool,
    },
    /// Starts tracking a show
    Track { id: String },
    /// Stops tracking a show
    Untrack { id: String },
    /// Updates the show list
    Update,
}
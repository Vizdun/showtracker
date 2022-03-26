use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Check if any new episodes premiered
    Check,
    /// List tracked shows
    List,
    /// Searche for a show
    Search {
        search_term: String,
        #[clap(default_value_t = 5)]
        max: u32,
    },
    /// Start tracking a show
    Track { id: String },
    /// Stop tracking a show
    Untrack { id: String },
    /// Update information about tracked shows
    Update,
}

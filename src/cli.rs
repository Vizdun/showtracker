use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Check if any new episodes premiered
    Check {
        /// Keep old information (notify until update is run)
        #[clap(short, long)]
        keep: bool,
    },
    /// List tracked shows
    List {
        /// Render the list as a tree
        #[clap(short, long)]
        tree: bool,
    },
    /// Search for a show
    Search {
        /// Search term
        search_term: String,
        /// Maximum number of results to show
        #[clap(default_value_t = 5)]
        max: u32,
    },
    /// Start tracking a show
    Track {
        /// ID or title
        show: String,
    },
    /// Stop tracking a show
    Untrack {
        /// ID or title
        show: String,
    },
    /// Update information about tracked shows
    Update,
}

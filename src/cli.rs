use clap::{Parser, Subcommand, ArgEnum};

#[derive(Parser)]
#[clap(version = env!("HASHVER"))]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Check if any new episodes aired
    Check {
        /// Keep old information (notify until update is run)
        #[clap(short, long)]
        keep: bool,
    },
    /// List tracked shows
    List {
        /// Show IDs
        #[clap(short, long)]
        id: bool,
        /// Render the list as a tree
        #[clap(short, long)]
        tree: bool,
        /// Numeral system
        #[clap(short, long, arg_enum, default_value_t = Numeral::Arabic)]
        numeral: Numeral,
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
    Update {
        /// Update one show specifically
        show: Option<String>,
        /// Update all seasons
        #[clap(short, long)]
        full: bool,
    },
}

#[derive(ArgEnum, Debug, Clone)] // ArgEnum here
#[clap(rename_all = "kebab_case")]
pub enum Numeral {
    Arabic,
    Roman,
    TallyMarks,
}

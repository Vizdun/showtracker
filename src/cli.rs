use clap::{Parser, Subcommand, ArgEnum};

#[derive(Parser)]
#[clap(version = env!("HASHVER"))]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Print out debug information
    #[cfg(debug_assertions)]
    Debug,
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
        #[clap(short, long, default_value_t = 5)]
        max: u32,
        /// Show IDs
        #[clap(short, long)]
        id: bool,
        /// Use stars to display rating
        #[clap(short, long)]
        stars: bool,
        /// Numeral system
        #[clap(short, long, arg_enum, default_value_t = Numeral::Arabic)]
        numeral: Numeral,
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

#[derive(ArgEnum, Debug, Clone)]
#[clap(rename_all = "kebab_case")]
pub enum Numeral {
    Arabic,
    Roman,
    TallyMarks,
}

use crate::{
    storage::load_tracked_shows,
    structs::display::TrackedShows, cli::Numeral,
};

pub fn list(
    id: bool,
    tree: bool,
    numeral: Numeral,
) {
    let track_list = load_tracked_shows();

    print!(
        "{}",
        TrackedShows {
            shows: track_list,
            id,
            tree,
            numeral
        }
    );
}

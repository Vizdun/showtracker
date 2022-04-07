use crate::storage::load_tracked_shows;

pub fn debug() {
    let shows = load_tracked_shows();

    print!("{:#?}", shows)
}

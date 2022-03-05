use crate::{
    storage::load_tracked_shows, structs::TrackedShows,
};

pub fn list() {
    let track_list = load_tracked_shows();

    println!("{}", TrackedShows(track_list));
}

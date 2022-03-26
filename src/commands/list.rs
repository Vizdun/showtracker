use ptree::print_tree;

use crate::{
    storage::load_tracked_shows,
    structs::display::TrackedShows,
};

pub fn list(tree: bool) {
    let track_list = load_tracked_shows();

    if tree {
        print_tree(&TrackedShows(track_list)).unwrap();
    } else {
        println!("{}", TrackedShows(track_list));
    }
}

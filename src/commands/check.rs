use crate::{storage::*, common::last_episode};

pub fn check() {
    let track_list = load_tracked_shows();

    for show in &track_list {
        let new_last = last_episode(&show.seasons);
        if new_last.1 > show.last_episode.1 {
            println!("New episode of {}", show.title);
        } else if new_last.0 > show.last_episode.0 {
            println!("New season of {}", show.title);
        }
    }

    save_tracked_shows(track_list);
}

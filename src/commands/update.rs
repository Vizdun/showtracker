use crate::{common::fetch_show, storage::*};

pub fn update() {
    let track_list = load_tracked_shows();

    let mut new_track_list = vec![];

    for show in track_list {
        new_track_list.push(fetch_show(
            show.id,
            show.last_episode.0 as u32 - 1,
        ));
    }

    save_tracked_shows(new_track_list);
}

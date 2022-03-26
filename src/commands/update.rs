use crate::{
    common::{fetch_show, parse_show_id},
    storage::*,
};

pub fn update(
    show: Option<String>,
    full: bool,
) {
    let track_list = match show {
        Some(s) => vec![load_tracked_shows()
            .iter()
            .find(|x| x.id == parse_show_id(&s))
            .unwrap()
            .clone()],
        None => load_tracked_shows(),
    };

    let mut new_track_list = vec![];

    if full {
        for show in track_list {
            new_track_list.push(fetch_show(show.id, 0));
        }
    } else {
        for show in track_list {
            new_track_list.push(fetch_show(
                show.id,
                show.last_episode.0 as u32 - 1,
            ));
        }
    }

    save_tracked_shows(new_track_list);
}

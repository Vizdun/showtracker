use crate::{common::check_shows, storage::*};

pub fn check() {
    let mut track_list = load_tracked_shows();

    let checked_shows = check_shows(
        &track_list
            .iter()
            .map(|x| x.id)
            .collect::<Vec<u32>>(),
    );

    track_list
        .sort_by(|a, b| a.id.partial_cmp(&b.id).unwrap());

    for (id, track) in checked_shows {
        let indx = track_list
            .binary_search_by(|probe| probe.id.cmp(&id))
            .unwrap();
        if track_list[indx].episode_count < track {
            println!(
                "New episode of {}",
                track_list[indx].name
            );
        }

        track_list[indx].episode_count = track;
    }

    save_tracked_shows(track_list);
}

use crate::{
    common::{parse_show_id, fetch_show},
    storage::*,
};

pub fn track(show: &str) {
    let id = parse_show_id(show);

    let mut track_list = load_tracked_shows();

    if track_list.iter().any(|s| s.id == id) {
        println!(
            "Already tracking {}",
            track_list
                .iter()
                .find(|s| s.id == id)
                .unwrap()
                .title
        );

        return;
    }

    let show = fetch_show(id, 0);

    println!("Added {} to tracked shows", show.title);

    track_list.push(show);

    save_tracked_shows(track_list);
}

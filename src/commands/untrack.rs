use crate::{common::parse_show_id, storage::*};

pub fn untrack(show: &str) {
    let id = parse_show_id(show);

    let mut track_list = load_tracked_shows();

    let index = match track_list
        .to_vec()
        .iter()
        .position(|item| item.id == id)
    {
        Some(index) => index,
        None => {
            panic!["Invalid ID"];
        }
    };

    println!(
        "Stopped tracking {}",
        track_list[index].title
    );

    track_list.remove(index);

    save_tracked_shows(track_list);
}

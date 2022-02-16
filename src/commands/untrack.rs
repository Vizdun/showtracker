use crate::{common::parse_show_id, storage::*};

pub fn untrack(show: &str) {
    let result = parse_show_id(show);

    let mut track_list = load_tracked_shows();

    let index = match track_list
        .to_vec()
        .iter()
        .position(|item| item.id == result.id)
    {
        Some(index) => index,
        None => {
            panic!["Invalid ID"];
        }
    };

    println!("Stopped tracking {}", track_list[index].name);

    track_list.remove(index);

    save_tracked_shows(track_list);
}

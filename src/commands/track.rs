use crate::{
    common::{parse_show_id, check_shows},
    storage::*,
    structs::*,
};

pub fn track(show: &str) {
    let result = parse_show_id(show);

    let mut track_list = load_tracked_shows();

    if (&track_list).iter().any(|item| item.id == result.id)
    {
        panic!["Invalid ID"];
    };

    track_list.push(TrackedShow {
        id: result.id,
        episode_count: check_shows(&[result.id])
            .first()
            .unwrap()
            .1,
        name: (&result.name).to_string(),
    });

    save_tracked_shows(track_list);

    println!("Added {} to tracked shows", result.name);
}

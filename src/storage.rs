use std::fs;

use home::home_dir;

use crate::structs::{Show, TrackedShow};

#[derive(serde::Serialize, serde::Deserialize)]
struct SerializedTrackedShows {
    tracked_shows: Vec<TrackedShow>,
}

pub fn load_show_list() -> Vec<Show> {
    let show_list_bin = fs::read(format!(
        "{}/.config/showtracker/shows.bin",
        home_dir().unwrap().display()
    ))
    .unwrap();

    let show_list: Vec<Show> =
        bincode::deserialize(&show_list_bin).unwrap();

    show_list
}

pub fn save_show_list(shows: Vec<Show>) {
    let shows_bin = bincode::serialize(&shows).unwrap();

    fs::write(
        format!(
            "{}/.config/showtracker/shows.bin",
            home_dir().unwrap().display()
        ),
        shows_bin,
    )
    .expect("Unable to write file");
}

pub fn load_tracked_shows() -> Vec<TrackedShow> {
    let track_list_bin = fs::read(format!(
        "{}/.config/showtracker/tracked_shows.toml",
        home_dir().unwrap().display()
    ))
    .unwrap();

    let serialized_tracked_shows: SerializedTrackedShows =
        toml::from_slice(&track_list_bin).unwrap();

    serialized_tracked_shows.tracked_shows
}

pub fn save_tracked_shows(track_list: Vec<TrackedShow>) {
    let serialized_tracked_shows = SerializedTrackedShows {
        tracked_shows: track_list,
    };

    let track_list_bin =
        toml::to_string_pretty(&serialized_tracked_shows)
            .unwrap();

    fs::create_dir_all(format!(
        "{}/.config/showtracker/",
        home_dir().unwrap().display()
    ))
    .expect("Unable to create .config directory");

    fs::write(
        format!(
            "{}/.config/showtracker/tracked_shows.toml",
            home_dir().unwrap().display()
        ),
        track_list_bin,
    )
    .expect("Unable to write file");
}

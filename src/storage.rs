use std::fs;

use crate::structs::TrackedShow;

#[derive(serde::Serialize, serde::Deserialize)]
struct SerializedTrackedShows {
    tracked_shows: Vec<TrackedShow>,
}

fn config_dir(file: &str) -> String {
    format!(
        "{}/.config/showtracker/{}",
        home::home_dir().unwrap().display(),
        file
    )
}

pub fn load_tracked_shows() -> Vec<TrackedShow> {
    match fs::read(config_dir("tracked_shows.toml")) {
        Ok(track_list_bin) => {
            let serialized_tracked_shows: SerializedTrackedShows =
        bincode::deserialize(&track_list_bin).unwrap();

            serialized_tracked_shows.tracked_shows
        }
        Err(_) => vec![],
    }
}

pub fn save_tracked_shows(track_list: Vec<TrackedShow>) {
    let serialized_tracked_shows = SerializedTrackedShows {
        tracked_shows: track_list,
    };

    let track_list_bin =
        bincode::serialize(&serialized_tracked_shows)
            .unwrap();

    fs::create_dir_all(config_dir(""))
        .expect("Unable to create .config directory");

    fs::write(
        config_dir("tracked_shows.toml"),
        track_list_bin,
    )
    .expect("Unable to write file");
}

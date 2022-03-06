use std::fs;

use crate::structs::{Show, TrackedShow};

#[derive(serde::Serialize, serde::Deserialize)]
struct SerializedTrackedShows {
    tracked_shows: Vec<SerializedTrackedShow>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct SerializedTrackedShow {
    id: Option<u32>,
    episode_count: Option<u16>,
    name: Option<String>,
}

impl From<&SerializedTrackedShow> for TrackedShow {
    fn from(show: &SerializedTrackedShow) -> Self {
        TrackedShow {
            id: show.id.unwrap_or(0),
            episode_count: show.episode_count.unwrap_or(0),
            name: show
                .name
                .clone()
                .unwrap_or_else(|| String::from("null")),
        }
    }
}

impl From<&TrackedShow> for SerializedTrackedShow {
    fn from(show: &TrackedShow) -> Self {
        SerializedTrackedShow {
            id: Some(show.id),
            episode_count: Some(show.episode_count),
            name: Some(show.name.clone()),
        }
    }
}

fn config_dir(file: &str) -> String {
    format!(
        "{}/.config/showtracker/{}",
        home::home_dir().unwrap().display(),
        file
    )
}

pub fn load_show_list() -> Vec<Show> {
    let show_list_bin =
        fs::read(config_dir("shows.bin")).unwrap();

    let show_list: Vec<Show> =
        bincode::deserialize(&show_list_bin).unwrap();

    show_list
}

pub fn save_show_list(shows: Vec<Show>) {
    let shows_bin = bincode::serialize(&shows).unwrap();

    fs::create_dir_all(config_dir(""))
        .expect("Unable to create .config directory");

    fs::write(config_dir("shows.bin"), shows_bin)
        .expect("Unable to write file");
}

pub fn load_tracked_shows() -> Vec<TrackedShow> {
    let track_list_bin =
        fs::read(config_dir("tracked_shows.toml")).unwrap();

    let serialized_tracked_shows: SerializedTrackedShows =
        toml::from_slice(&track_list_bin).unwrap();

    serialized_tracked_shows
        .tracked_shows
        .iter()
        .map(TrackedShow::from)
        .collect()
}

pub fn save_tracked_shows(track_list: Vec<TrackedShow>) {
    let serialized_tracked_shows = SerializedTrackedShows {
        tracked_shows: track_list
            .iter()
            .map(SerializedTrackedShow::from)
            .collect(),
    };

    let track_list_bin =
        toml::to_string_pretty(&serialized_tracked_shows)
            .unwrap();

    fs::create_dir_all(config_dir(""))
        .expect("Unable to create .config directory");

    fs::write(
        config_dir("tracked_shows.toml"),
        track_list_bin,
    )
    .expect("Unable to write file");
}

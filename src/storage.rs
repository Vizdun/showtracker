use home::home_dir;
use std::convert::TryInto;
use std::fs;

use crate::structs::{Show, TrackedShow};

pub fn load_show_list() -> Vec<Show> {
  let mut show_list: Vec<Show> = vec![];

  let show_list_bin = fs::read(format!(
    "{}/.config/showtracker/shows.bin",
    home_dir().unwrap().display()
  ))
  .unwrap();

  let mut index = 0;

  while index < show_list_bin.len() {
    let last_i =
      index + 12 + usize::from_le_bytes(show_list_bin[index + 4..index + 12].try_into().unwrap());
    show_list.push(Show {
      id: u32::from_le_bytes(show_list_bin[index + 0..index + 4].try_into().unwrap()),
      name: String::from_utf8(show_list_bin[index + 12..last_i].to_vec()).unwrap(),
    });
    index = last_i;
  }

  return show_list;
}

pub fn save_show_list(shows: Vec<Show>) {
  let shows_bin = shows
    .into_iter()
    .map(|show| {
      show
        .id
        .to_le_bytes()
        .iter()
        .chain(show.name.as_bytes().len().to_le_bytes().iter())
        .chain(show.name.as_bytes().into_iter())
        .map(|x| x.to_owned())
        .collect::<Vec<u8>>()
    })
    .collect::<Vec<Vec<u8>>>()
    .concat();

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
  let mut track_list: Vec<TrackedShow> = vec![];

  let track_list_bin = fs::read(format!(
    "{}/.config/showtracker/tracked_shows.bin",
    home_dir().unwrap().display()
  ))
  .unwrap();

  let mut index = 0;

  while index < track_list_bin.len() {
    let last_i =
      index + 14 + usize::from_le_bytes(track_list_bin[index + 6..index + 14].try_into().unwrap());
    track_list.push(TrackedShow {
      id: u32::from_le_bytes(track_list_bin[index + 0..index + 4].try_into().unwrap()),
      episode_count: u16::from_le_bytes(track_list_bin[index + 4..index + 6].try_into().unwrap()),
      name: String::from_utf8(track_list_bin[index + 14..last_i].to_vec()).unwrap(),
    });
    index = last_i;
  }

  return track_list;
}

pub fn save_tracked_shows(track_list: Vec<TrackedShow>) {
  let track_list_bin = track_list
    .into_iter()
    .map(|tracked_show| {
      tracked_show
        .id
        .to_le_bytes()
        .iter()
        .chain(tracked_show.episode_count.to_le_bytes().iter())
        .chain(tracked_show.name.as_bytes().len().to_le_bytes().iter())
        .chain(tracked_show.name.as_bytes().into_iter())
        .map(|x| x.to_owned())
        .collect::<Vec<u8>>()
    })
    .collect::<Vec<Vec<u8>>>()
    .concat();

  fs::create_dir_all(format!(
    "{}/.config/showtracker/",
    home_dir().unwrap().display()
  ))
  .expect("Unable to create .config directory");

  fs::write(
    format!(
      "{}/.config/showtracker/tracked_shows.bin",
      home_dir().unwrap().display()
    ),
    track_list_bin,
  )
  .expect("Unable to write file");
}

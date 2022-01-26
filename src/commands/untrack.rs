use crate::commands::common::parse_show_id;
use crate::storage::*;
use clap::{Error, ErrorKind};

pub fn untrack_show(show: &str) {
  let result = parse_show_id(show);
  let mut track_list = load_tracked_shows();

  let index = match track_list
    .to_vec()
    .into_iter()
    .position(|item| item.id == result.id)
  {
    Some(index) => index,
    None => Error::with_description("Show not tracked", ErrorKind::InvalidValue).exit(),
  };
  println!("Stopped tracking {}", track_list[index].name);
  track_list.remove(index);
  save_tracked_shows(track_list);
}

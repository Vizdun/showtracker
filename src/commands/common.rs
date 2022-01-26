use crate::storage::*;
use crate::structs::*;
use clap::{Error, ErrorKind};

pub fn parse_show_id(show: &str) -> Show {
  let id = match u32::from_str_radix(show, 16) {
    Ok(num) => num,
    Err(_) => Error::with_description("Invalid ID", ErrorKind::InvalidValue).exit(),
  };

  let shows = load_show_list();
  return match shows.into_iter().find(|item| item.id == id) {
    Some(result) => result,
    None => Error::with_description("Show not found", ErrorKind::InvalidValue).exit(),
  };
}

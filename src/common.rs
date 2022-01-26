const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/50.0.2661.102 Safari/537.36";

pub fn get_request(url: &str) -> String {
    reqwest::blocking::Client::new()
        .get(url)
        .header("User-Agent", USER_AGENT)
        .send()
        .unwrap()
        .text()
        .unwrap()
}

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

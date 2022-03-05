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

use crate::{storage::*, structs::*};

pub fn parse_show_id(show: &str) -> Show {
    let id: u32 = match bs58::decode(show).into_vec() {
        Ok(vec) => {
            u32::from_be_bytes(vec.try_into().unwrap())
        }
        Err(_) => panic!["Invalid ID"],
    };

    let shows = load_show_list();

    match shows.into_iter().find(|item| item.id == id) {
        Some(result) => result,
        None => panic!["Invalid ID"],
    }
}

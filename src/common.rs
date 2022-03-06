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

use scraper::{Html, Selector};

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

fn query(track_list: &[u32]) -> String {
    format!(
        include_str!("queries/check.sparql"),
        track_list
            .iter()
            .map(|track| format!("wd:Q{}", track))
            .collect::<Vec<String>>()
            .join(" ")
    )
}

pub fn check_shows(track_list: &[u32]) -> Vec<(u32, u16)> {
    let xml = get_request(&format!(
        "https://query.wikidata.org/sparql?query={}",
        urlencoding::encode(&query(track_list))
    ));

    let fragment = Html::parse_fragment(&xml);
    let result_selector =
        Selector::parse("result").unwrap();
    let show_id_selector =
        Selector::parse("binding[name=show] uri").unwrap();
    let episode_count_selector = Selector::parse(
        "binding[name=episodeCount] literal",
    )
    .unwrap();

    fragment
        .select(&result_selector)
        .map(|x| {
            (
                x.select(&show_id_selector)
                    .next()
                    .unwrap()
                    .text()
                    .collect::<String>()
                    .split_once('Q')
                    .unwrap()
                    .1
                    .parse::<u32>()
                    .unwrap(),
                x.select(&episode_count_selector)
                    .next()
                    .unwrap()
                    .text()
                    .collect::<String>()
                    .parse::<u16>()
                    .unwrap(),
            )
        })
        .collect::<Vec<(u32, u16)>>()
}

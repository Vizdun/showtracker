use crate::commands::common::parse_show_id;
use crate::constants::*;
use crate::storage::*;
use crate::structs::*;
use clap::{Error, ErrorKind};

pub fn track_show(show: &str) {
  let result = parse_show_id(show);

  let mut track_list = load_tracked_shows();

  match (&track_list).into_iter().find(|item| item.id == result.id) {
    Some(_) => Error::with_description("Show already tracked", ErrorKind::InvalidValue).exit(),
    None => {}
  };

  track_list.push(TrackedShow {
      id: result.id,
      episode_count: reqwest::blocking::Client::new().get(format!(
          "https://query.wikidata.org/sparql?query=SELECT%20%3FepisodeCount%0AWHERE%0A%7B%0A%20%20wd%3AQ{}%20wdt%3AP1113%20%3FepisodeCount.%0A%7D",
          result.id
      ))
      .header("User-Agent", USER_AGENT)
      .send()
      .unwrap()
      .text()
      .unwrap()
      .splitn(2, "<literal datatype='http://www.w3.org/2001/XMLSchema#decimal'>")
      .collect::<Vec<&str>>()[1]
      .splitn(2, "</literal>")
      .collect::<Vec<&str>>()[0]
      .parse::<u16>()
      .unwrap(),
      name: (&result.name).to_string(),
  });

  save_tracked_shows(track_list);
  println!("Added {} to tracked shows", result.name);
}

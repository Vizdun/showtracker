extern crate clap;
use clap::{App, AppSettings, Arg, Error, ErrorKind};
use home::home_dir;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;

const QUERY_URL: &str = "https://query.wikidata.org/sparql?query=SELECT%0A%20%20%3Fitem%20%3FitemLabel%0AWHERE%20%0A%7B%0A%20%20%3Fitem%20wdt%3AP1113%20%3Fvalue.%0A%20%20SERVICE%20wikibase%3Alabel%20%7B%20bd%3AserviceParam%20wikibase%3Alanguage%20%22en%22.%20%7D%0A%7D";

async fn get_episode_count(id: u32) -> u32 {
    let body = reqwest::get(format!(
        "https://www.wikidata.org/wiki/Special:EntityData/Q{}.json",
        id
    ))
    .await
    .unwrap()
    .text()
    .await
    .unwrap();

    return body.splitn(2, "P1113").collect::<Vec<&str>>()[1]
        .splitn(2, ",\"unit")
        .collect::<Vec<&str>>()[0]
        .splitn(2, "amount\":\"+")
        .collect::<Vec<&str>>()[1]
        .splitn(2, "\"")
        .collect::<Vec<&str>>()[0]
        .parse::<u32>()
        .unwrap();
}

fn update_show_list() {
    let client = reqwest::blocking::Client::new();

    let shows = client.get(QUERY_URL)
    .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/50.0.2661.102 Safari/537.36")
    .send()
    .unwrap()
    .text()
    .unwrap()
    .split("<result>")
    .collect::<Vec<&str>>()[1..]
    .into_iter()
    .map(|result| {
            let name_arr = result
            .splitn(2, "<literal xml:lang='en'>")
            .collect::<Vec<&str>>();
            let id = result
            .splitn(2, "http://www.wikidata.org/entity/Q")
            .collect::<Vec<&str>>()[1]
            .splitn(2, "</uri>")
            .collect::<Vec<&str>>()[0]
            .parse::<u32>()
            .unwrap();
            return Show {
                id: id,
                name: if name_arr.len() > 1 {
                    name_arr[1].splitn(2, "</literal>")
                    .collect::<Vec<&str>>()[0]
                    .to_string()
                } else {
                    id.to_string()
                }
            }
       }
    )
    .collect::<Vec<Show>>();

    let json = (json!(shows)).to_string();

    fs::create_dir_all(format!(
        "{}/.config/showtracker/",
        home_dir().unwrap().display()
    ))
    .expect("Unable to create .config directory");
    fs::write(
        format!(
            "{}/.config/showtracker/shows.json",
            home_dir().unwrap().display()
        ),
        json,
    )
    .expect("Unable to write file");
}

fn load_show_list() -> Vec<Show> {
    return serde_json::from_str(
        &fs::read_to_string(format!(
            "{}/.config/showtracker/shows.json",
            home_dir().unwrap().display()
        ))
        .unwrap_or("[]".to_owned())
        .to_string(),
    )
    .unwrap();
}

fn load_tracked_shows() -> Vec<TrackedShow> {
    return serde_json::from_str(
        &fs::read_to_string(format!(
            "{}/.config/showtracker/tracked_shows.json",
            home_dir().unwrap().display()
        ))
        .unwrap_or("[]".to_owned())
        .to_string(),
    )
    .unwrap();
}

fn save_tracked_shows(track_list: Vec<TrackedShow>) {
    fs::create_dir_all(format!(
        "{}/.config/showtracker/",
        home_dir().unwrap().display()
    ))
    .expect("Unable to create .config directory");
    let json = (json!(track_list)).to_string();
    fs::write(
        format!(
            "{}/.config/showtracker/tracked_shows.json",
            home_dir().unwrap().display()
        ),
        json,
    )
    .expect("Unable to write file");
}

async fn check_for_new_episode(i: usize, track: TrackedShow) -> (usize, u32) {
    let new_episode_count = get_episode_count(track.id).await;
    if track.episode_count < new_episode_count {
        println!("New episode of {}", track.name);
    }
    return (i, new_episode_count);
}

async fn check_for_new_episodes() {
    let mut track_list = load_tracked_shows();

    let futs = (&track_list)
        .to_vec()
        .into_iter()
        .enumerate()
        .map(|track| return check_for_new_episode(track.0, track.1));
    
    for track_tuple in futures::future::join_all(futs).await {
        track_list[track_tuple.0].episode_count = track_tuple.1;
    }

    save_tracked_shows(track_list);
}

fn search_shows(show: &str) {
    let search_results = load_show_list()
        .into_iter()
        .filter(|vec_show| vec_show.name.to_lowercase().contains(&show.to_lowercase()))
        .collect::<Vec<Show>>();
    for result in search_results {
        println!("{:07x} {}", result.id, result.name);
    }
}

fn parse_show_id(show: &str) -> Show {
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

async fn track_show(show: &str) {
    let result = parse_show_id(show);

    let mut track_list = load_tracked_shows();

    match (&track_list).into_iter().find(|item| item.id == result.id) {
        Some(_) => Error::with_description("Show already tracked", ErrorKind::InvalidValue).exit(),
        None => {}
    };

    track_list.push(TrackedShow {
        id: result.id,
        episode_count: get_episode_count(result.id).await,
        name: (&result.name).to_string(),
    });

    save_tracked_shows(track_list);
    println!("Added {} to tracked shows", result.name);
}

fn untrack_show(show: &str) {
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

fn list_tracked() {
    let track_list = load_tracked_shows();
    for track in track_list {
        println!("{:07x} {}", track.id, track.name);
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Show {
    id: u32,
    name: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct TrackedShow {
    id: u32,
    episode_count: u32,
    name: String,
}

#[tokio::main]
async fn main() {
    let matches = App::new("Show Tracker")
        .version("1.0.0")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(App::new("check").about("Checks if there are any new episodes"))
        .subcommand(App::new("update").about("Updates the show list"))
        .subcommand(
            App::new("search").about("Searches the show list").arg(
                Arg::with_name("TERM")
                    .help("Search term")
                    .required(true)
                    .index(1),
            ),
        )
        .subcommand(
            App::new("track").about("Starts tracking a show").arg(
                Arg::with_name("SHOW")
                    .help("Show ID")
                    .required(true)
                    .index(1),
            ),
        )
        .subcommand(
            App::new("untrack").about("Stops tracking a show").arg(
                Arg::with_name("SHOW")
                    .help("Show ID")
                    .required(true)
                    .index(1),
            ),
        )
        .subcommand(App::new("list").about("Lists tracked shows"))
        .get_matches();

    match matches.subcommand().0 {
        "update" => update_show_list(),
        "search" => search_shows(
            matches
                .subcommand_matches("search")
                .unwrap()
                .value_of("TERM")
                .unwrap(),
        ),
        "track" => track_show(
            matches
                .subcommand_matches("track")
                .unwrap()
                .value_of("SHOW")
                .unwrap(),
        ).await,
        "untrack" => untrack_show(
            matches
                .subcommand_matches("untrack")
                .unwrap()
                .value_of("SHOW")
                .unwrap(),
        ),
        "list" => list_tracked(),
        "check" => check_for_new_episodes().await,
        _ => {}
    }
}

use crate::{
    common::{get_request, parse_show_id},
    storage::*,
    structs::*,
};

pub fn main(show: &str) {
    let result = parse_show_id(show);

    let mut track_list = load_tracked_shows();

    if (&track_list).iter().any(|item| item.id == result.id)
    {
        panic!["Invalid ID"];
    };

    track_list.push(TrackedShow {
        id: result.id,
        episode_count: get_request(&format!(
            "https://query.wikidata.org/sparql?query=SELECT%20%3FepisodeCount%0AWHERE%0A%7B%0A%20%20wd%3AQ{}%20wdt%3AP1113%20%3FepisodeCount.%0A%7D",
            result.id
        ))
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

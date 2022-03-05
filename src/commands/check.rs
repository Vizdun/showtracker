use scraper::{Html, Selector};

use crate::{
    common::get_request, storage::*, structs::TrackedShow,
};

fn query(track_list: &[TrackedShow]) -> String {
    format!(
        r#"SELECT ?show ?episodeCount
WHERE
{{
  VALUES ?show {{{}}}.
  ?show wdt:P1113 ?episodeCount.
}}"#,
        track_list
            .iter()
            .map(|track| format!("wd:Q{}", track.id))
            .collect::<Vec<String>>()
            .join(" ")
    )
}

pub fn check() {
    let mut track_list = load_tracked_shows();

    let xml = get_request(&format!(
        "https://query.wikidata.org/sparql?query={}",
        urlencoding::encode(&query(&track_list))
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

    let checked_shows = fragment
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
        .collect::<Vec<(u32, u16)>>();

    track_list
        .sort_by(|a, b| a.id.partial_cmp(&b.id).unwrap());

    for (id, track) in checked_shows {
        let indx = track_list
            .binary_search_by(|probe| probe.id.cmp(&id))
            .unwrap();
        if track_list[indx].episode_count < track {
            println!(
                "New episode of {}",
                track_list[indx].name
            );
        }

        track_list[indx].episode_count = track;
    }

    save_tracked_shows(track_list);
}

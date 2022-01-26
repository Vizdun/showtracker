use crate::common::get_request;
use crate::storage::*;

pub fn check_for_new_episodes() {
    let mut track_list = load_tracked_shows();
    let checked_shows = get_request(format!("https://query.wikidata.org/sparql?query=SELECT%20%3FepisodeCount%0AWHERE%0A%7B%0A%20%20VALUES%20%3Fshow%20%7Bwd%3A{}%0A%20%20%3Fshow%20wdt%3AP1113%20%3FepisodeCount.%0A%7D",(&track_list).into_iter().map(|track| track.id.to_string()).collect::<Vec<String>>().join("%20wd%3A"))).split("<literal datatype='http://www.w3.org/2001/XMLSchema#decimal'>").collect::<Vec<&str>>()[1..].into_iter().enumerate().map(|(id, result)| {return (id, result.splitn(2, "</literal>")                                                                .collect::<Vec<&str>>()[0]                                                                .parse::<u16>()                                                            .unwrap())                                                    })                                                .collect::<Vec<(usize, u16)>>();
    
    for track_tuple in checked_shows {
        if track_list[track_tuple.0].episode_count < track_tuple.1 {
            println!("New episode of {}", track_list[track_tuple.0].name);
        }
        track_list[track_tuple.0].episode_count = track_tuple.1;
    }
    
    save_tracked_shows(track_list);
}

use crate::{common::get_request, storage::*, structs::*};

use urlencoding::encode;

pub fn update() {
    let query_url = &format!(
        "https://query.wikidata.org/sparql?query={}",
        encode(
            r#"SELECT
?item ?itemLabel ?date
WHERE
{
?item wdt:P31 wd:Q5398426.
?item wdt:P1113 ?episodeCount.
OPTIONAL {?item wdt:P577 ?date}
OPTIONAL {?item wdt:P580 ?date}
SERVICE wikibase:label { bd:serviceParam wikibase:language "en". }
}"#
        )
    );

    let shows = get_request(query_url)
        .split("<result>")
        .collect::<Vec<&str>>()[1..]
        .iter()
        .map(|result| -> Show {
            let name_arr = result
                .splitn(2, "<literal xml:lang='en'>")
                .collect::<Vec<&str>>();

            let id = result
                .splitn(
                    2,
                    "http://www.wikidata.org/entity/Q",
                )
                .collect::<Vec<&str>>()[1]
                .splitn(2, "</uri>")
                .collect::<Vec<&str>>()[0]
                .parse::<u32>()
                .unwrap();

            let date_arr = result.splitn(2, "<literal datatype='http://www.w3.org/2001/XMLSchema#dateTime'>").collect::<Vec<&str>>();

            return Show {
                id,
                name: if name_arr.len() > 1 {
                    name_arr[1]
                        .splitn(2, "</literal>")
                        .collect::<Vec<&str>>()[0]
                        .to_string()
                } else {
                    id.to_string()
                },
                year: if date_arr.len() > 1 {
                    let date_small = date_arr[1]
                        .splitn(2, "</literal>")
                        .collect::<Vec<&str>>()[0]
                        [0..4]
                        .parse::<u32>()
                        .unwrap();

                    if !(1880..=2135).contains(&date_small) {
                        0
                    } else {
                        (date_small - 1880) as u8
                    }
                } else {
                    0
                }
            }
        })
        .collect::<Vec<Show>>();

    save_show_list(shows);
}

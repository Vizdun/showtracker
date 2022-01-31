use crate::{common::get_request, storage::*, structs::*};

use urlencoding::encode;

pub fn update_show_list() {
    let query_url = &format!("https://query.wikidata.org/sparql?query={}",encode("SELECT
?item ?itemLabel
WHERE 
{
?item wdt:P1113 ?value.
MINUS
{
  ?item wdt:P31 wd:Q3464665.
}
MINUS
{
  ?item wdt:P31 wd:Q100269041.
}
SERVICE wikibase:label { bd:serviceParam wikibase:language \"en\". }
}"));

    let shows = get_request(query_url)
        .split("<result>")
        .collect::<Vec<&str>>()[1..]
        .into_iter()
        .map(|result| {
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
            };
        })
        .collect::<Vec<Show>>();

    save_show_list(shows);
}

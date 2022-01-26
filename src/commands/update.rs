use crate::common::get_request;
use crate::storage::*;
use crate::structs::*;

pub const QUERY_URL: &str = "https://query.wikidata.org/sparql?query=SELECT%0A%20%20%3Fitem%20%3FitemLabel%0AWHERE%20%0A%7B%0A%20%20%3Fitem%20wdt%3AP1113%20%3Fvalue.%0A%20%20SERVICE%20wikibase%3Alabel%20%7B%20bd%3AserviceParam%20wikibase%3Alanguage%20%22en%22.%20%7D%0A%7D";

pub fn update_show_list() {
    let shows = get_request(QUERY_URL)
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
                    name_arr[1].splitn(2, "</literal>").collect::<Vec<&str>>()[0].to_string()
                } else {
                    id.to_string()
                },
            };
        })
        .collect::<Vec<Show>>();

    save_show_list(shows);
}

use crate::constants::*;
use crate::storage::*;
use crate::structs::*;

pub async fn update_show_list() {
    let client = reqwest::Client::new();

    let shows = client
        .get(QUERY_URL)
        .header("User-Agent", USER_AGENT)
        .send()
        .await
        .unwrap()
        .text()
        .await
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
                    name_arr[1].splitn(2, "</literal>").collect::<Vec<&str>>()[0].to_string()
                } else {
                    id.to_string()
                },
            };
        })
        .collect::<Vec<Show>>();

    save_show_list(shows);
}

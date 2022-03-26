use crate::structs::{Show, Shows};

use scraper::{Html, Selector};

pub fn search(
    search_query: &str,
    max_results: &u32,
) {
    let body = reqwest::blocking::Client::new()
        .get(format!("https://www.imdb.com/search/title/?title={}&title_type=tv_series,tv_miniseries,tv_short,podcast_series&adult=include", urlencoding::encode(search_query)))
        .header("Accept-Language", "en-US,en;q=0.5")
        .send()
        .unwrap()
        .text()
        .unwrap();

    let show_selector =
        Selector::parse("h3.lister-item-header").unwrap();

    let title_id_selector = Selector::parse("a").unwrap();
    let year_selector = Selector::parse(
        "span.lister-item-year.text-muted.unbold",
    )
    .unwrap();

    let fragment = Html::parse_fragment(&body);

    let search_results = fragment
        .select(&show_selector)
        .map(|e| {
            let title_id = e
                .select(&title_id_selector)
                .next()
                .unwrap();

            let title =
                title_id.text().next().unwrap().to_string();

            let id = title_id.value().attr("href").unwrap()
                [9..]
                .split_once('/')
                .unwrap()
                .0
                .parse::<u32>()
                .unwrap();

            let year = e
                .select(&year_selector)
                .next()
                .unwrap()
                .text()
                .next()
                .unwrap()[1..5]
                .parse::<u16>()
                .unwrap();

            Show { id, title, year }
        })
        .collect::<Vec<Show>>();

    let max_results: usize = *max_results as usize;

    let len = if search_results.len() > max_results {
        max_results
    } else {
        search_results.len()
    };

    println!("{}", Shows(search_results[0..len].to_vec()));
}

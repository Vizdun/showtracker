use chrono::{Utc, DateTime, NaiveDate};
use scraper::{Html, Selector};

use crate::{
    structs::{Episode, TrackedShow, Show},
};

pub fn get_request(url: String) -> String {
    reqwest::blocking::Client::new()
        .get(url)
        .header("Accept-Language", "en-US,en;q=0.5")
        .send()
        .unwrap()
        .text()
        .unwrap()
}

pub fn parse_show_id(show: &str) -> u32 {
    match bs58::decode(show).into_vec() {
        Ok(vec) => {
            u32::from_be_bytes(vec.try_into().unwrap())
        }
        Err(_) => search_shows(show).first().unwrap().id,
    }
}

pub fn last_episode(
    seasons: &[Vec<Episode>]
) -> (usize, usize) {
    let mut last_episode: (usize, usize) = (0, 0);

    for (s_i, season) in seasons.iter().enumerate() {
        for (e_i, episode) in season.iter().enumerate() {
            if episode.premier < Utc::now() {
                last_episode = (s_i + 1, e_i + 1);
            }
        }
    }

    last_episode
}

pub fn fetch_show(id: u32) -> TrackedShow {
    let mut body = get_request(format!(
        "https://www.imdb.com/title/tt{:#07}/episodes?season=1",
        id
    ));

    let mut fragment = Html::parse_fragment(&body);

    let show_title_selector =
        Selector::parse("h3[itemprop=name] > a").unwrap();

    let episode_selector =
        Selector::parse("div.info").unwrap();

    let title_selector =
        Selector::parse("strong > a").unwrap();
    let premier_selector =
        Selector::parse("div.airdate").unwrap();

    let season_nums =
        Selector::parse("select#bySeason > option")
            .unwrap();

    let title = fragment
        .select(&show_title_selector)
        .next()
        .unwrap()
        .text()
        .next()
        .unwrap()
        .to_string();

    let number_of_seasons = fragment
        .select(&season_nums)
        .map(|x| {
            x.text().next().unwrap().trim().parse::<u32>()
        })
        .filter(|x| match x {
            Ok(_) => true,
            Err(_) => false,
        })
        .last()
        .unwrap()
        .unwrap();

    let mut seasons: Vec<Vec<Episode>> = vec![];

    for i in 1..number_of_seasons + 1 {
        if i != 1 {
            body = get_request(format!("https://www.imdb.com/title/tt{:#07}/episodes?season={}", id, i));

            fragment = Html::parse_fragment(&body);
        }

        seasons.push(
            fragment
                .select(&episode_selector)
                .map(|e| {
                    let title = e
                        .select(&title_selector)
                        .next()
                        .unwrap()
                        .text()
                        .next()
                        .unwrap()
                        .to_string();

                    let mut premier = e
                        .select(&premier_selector)
                        .next()
                        .unwrap()
                        .text()
                        .next()
                        .unwrap()
                        .trim()
                        .split(' ')
                        .rev();

                    let year = premier
                        .next()
                        .unwrap()
                        .parse::<i32>()
                        .unwrap_or(9999);

                    let month = match premier
                        .next()
                        .unwrap_or("Dec.")
                    {
                        "Jan." => 1,
                        "Feb." => 2,
                        "Mar." => 3,
                        "Apr." => 4,
                        "May" => 5,
                        "Jun." => 6,
                        "Jul." => 7,
                        "Aug." => 8,
                        "Sep." => 9,
                        "Oct." => 10,
                        "Nov." => 11,
                        "Dec." => 12,
                        _ => panic!(),
                    };

                    let day = premier
                        .next()
                        .unwrap_or("28")
                        .parse::<u32>()
                        .unwrap();

                    Episode {
                        title,
                        premier: DateTime::<Utc>::from_utc(
                            NaiveDate::from_ymd(
                                year, month, day,
                            )
                            .and_hms(0, 0, 0),
                            Utc,
                        ),
                    }
                })
                .collect::<Vec<Episode>>(),
        );
    }

    let last_episode = last_episode(&seasons);

    TrackedShow { id, title, last_episode, seasons }
}

pub fn search_shows(search_query: &str) -> Vec<Show> {
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

    search_results
}

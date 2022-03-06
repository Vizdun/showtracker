use crate::{storage::*};

use crate::structs::{Show, Shows};

use regex::Regex;

pub fn search(
    search_query: &str,
    max_results: &u32,
    regex: bool,
) {
    let search_results = match regex {
        true => {
            let re = match Regex::new(search_query) {
                Err(regex::Error::Syntax(e)) => {
                    println!("{}", e);
                    return;
                }
                r => r.unwrap(),
            };

            load_show_list()
                .into_iter()
                .filter(|vec_show| {
                    re.is_match(&vec_show.name)
                })
                .collect::<Vec<Show>>()
        }
        false => load_show_list()
            .into_iter()
            .filter(|vec_show| {
                vec_show
                    .name
                    .to_lowercase()
                    .contains(&search_query.to_lowercase())
            })
            .collect::<Vec<Show>>(),
    };

    let max_results: usize = *max_results as usize;

    let len = if search_results.len() > max_results {
        max_results
    } else {
        search_results.len()
    };

    println!("{}", Shows(search_results[0..len].to_vec()));
}

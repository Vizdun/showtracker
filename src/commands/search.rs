use crate::{structs::Shows, common::search_shows};

pub fn search(
    search_query: &str,
    max_results: &u32,
) {
    let search_results = search_shows(search_query);

    let max_results: usize = *max_results as usize;

    let len = if search_results.len() > max_results {
        max_results
    } else {
        search_results.len()
    };

    println!("{}", Shows(search_results[0..len].to_vec()));
}

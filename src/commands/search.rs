use crate::{
    common::search_shows, structs::display::Shows,
    cli::Numeral,
};

pub fn search(
    search_query: &str,
    max_results: &u32,
    id: bool,
    stars: bool,
    numeral: Numeral,
) {
    let search_results = search_shows(search_query);

    let max_results: usize = *max_results as usize;

    let len = if search_results.len() > max_results {
        max_results
    } else {
        search_results.len()
    };

    println!(
        "{}",
        Shows {
            shows: search_results[0..len].to_vec(),
            id,
            stars,
            numeral
        }
    );
}

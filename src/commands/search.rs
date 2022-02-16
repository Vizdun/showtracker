use crate::{storage::*};

use crate::structs::{ShowsPrintable, ShowPrintable, Show};

pub fn search(
    search_query: &str,
    max_results: &u32,
) {
    let search_results = load_show_list()
        .into_iter()
        .filter(|vec_show| {
            vec_show
                .name
                .to_lowercase()
                .contains(&search_query.to_lowercase())
        })
        .collect::<Vec<Show>>();

    let max_results: usize = *max_results as usize;

    let len = if search_results.len() > max_results {
        max_results
    } else {
        search_results.len()
    };

    println!(
        "{}",
        ShowsPrintable {
            shows: search_results[0..len]
                .iter()
                .map(|result| {
                    ShowPrintable {
                        id: result.id,
                        name: result.name.clone(),
                        year: result.year,
                    }
                })
                .collect::<Vec<ShowPrintable>>(),
            years: true
        }
    );
}

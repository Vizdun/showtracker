use crate::{
    storage::*,
    structs::*
};

pub fn search_shows(show: &str) {

    let search_results = load_show_list()
        .into_iter()
        .filter(|vec_show| {
            vec_show
                .name
                .to_lowercase()
                .contains(&show.to_lowercase())
        })
        .collect::<Vec<Show>>();

    for result in search_results {

        println!("{:07x} {}", result.id, result.name);
    }
}

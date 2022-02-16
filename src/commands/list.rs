use crate::storage::load_tracked_shows;

use crate::structs::{ShowsPrintable, ShowPrintable};

pub fn main() {
    let track_list = load_tracked_shows();

    println!(
        "{}",
        ShowsPrintable {
            shows: track_list
                .into_iter()
                .map(|track| ShowPrintable {
                    id: track.id,
                    name: track.name,
                    year: 0
                })
                .collect(),
            years: false
        }
    );
}

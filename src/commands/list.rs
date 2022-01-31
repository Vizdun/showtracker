use crate::storage::load_tracked_shows;

use crate::structs::{ShowsPrintable, ShowPrintable};

pub fn list_tracked() {
    let track_list = load_tracked_shows();

    println!(
        "{}",
        ShowsPrintable {
            shows: track_list
                .into_iter()
                .map(|track| ShowPrintable {
                    id: track.id,
                    name: track.name
                })
                .collect()
        }
    );
}

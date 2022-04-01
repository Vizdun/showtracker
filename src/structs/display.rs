use comfy_table::{presets::NOTHING, Table};

use super::{TrackedShow, Show};

pub struct Shows(pub Vec<Show>);

impl std::fmt::Display for Shows {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let mut table = Table::new();

        table.load_preset(NOTHING);
        table.set_header(vec![
            "ID", "Title", "Year", "Rating",
        ]);

        for show in &self.0 {
            table.add_row(vec![
                bs58::encode(show.id.to_be_bytes())
                    .into_string(),
                show.title.clone(),
                show.year.map(|y| y.to_string())
                    .unwrap_or("".to_string()),
                show.rating.map(|r| r.to_string())
                    .unwrap_or("".to_string()),
            ]);
        }

        write!(f, "{table}")
    }
}

#[derive(Clone)]
pub struct TrackedShows(pub Vec<TrackedShow>);

impl std::fmt::Display for TrackedShows {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let mut table = Table::new();

        table.load_preset(NOTHING);
        table.set_header(vec![
            "ID", "Title", "Season", "Episode",
        ]);

        let mut shows = self.0.clone();

        shows.sort_by(|a, b| a.title.cmp(&b.title));

        for show in shows {
            table.add_row(vec![
                bs58::encode(show.id.to_be_bytes())
                    .into_string(),
                show.title.clone(),
                show.last_episode.0.to_string(),
                show.last_episode.1.to_string(),
            ]);
        }

        write!(f, "{table}")
    }
}

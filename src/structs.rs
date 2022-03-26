#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Show {
    pub id: u32,
    pub title: String,
    pub year: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    pub title: String,
    pub premier: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackedShow {
    pub id: u32,
    pub title: String,
    pub last_episode: (usize, usize),
    pub seasons: Vec<Vec<Episode>>,
}

use chrono::{Utc, DateTime};
use comfy_table::{Table, presets::NOTHING};
use serde::{Serialize, Deserialize};

pub struct Shows(pub Vec<Show>);

impl std::fmt::Display for Shows {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let mut table = Table::new();

        table.load_preset(NOTHING);
        table.set_header(vec!["ID", "Title", "Year"]);

        for show in &self.0 {
            table.add_row(vec![
                bs58::encode(show.id.to_be_bytes())
                    .into_string(),
                show.title.clone(),
                show.year.to_string(),
            ]);
        }

        write!(f, "{table}")
    }
}

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

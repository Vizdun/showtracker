#[derive(
    Debug, serde::Serialize, serde::Deserialize, Clone,
)]
pub struct Show {
    pub id: u32,
    pub name: String,
    pub year: u8,
}

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize,
)]
pub struct TrackedShow {
    pub id: u32,
    pub episode_count: u16,
    pub name: String,
}

use comfy_table::Table;

pub struct Shows(pub Vec<Show>);

impl std::fmt::Display for Shows {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let mut table = Table::new();

        table.set_header(vec!["ID", "Title", "Year"]);

        for show in &self.0 {
            table.add_row(vec![
                bs58::encode(show.id.to_be_bytes())
                    .into_string(),
                show.name.clone(),
                (1880 + show.year as u32).to_string(),
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

        table.set_header(vec![
            "ID",
            "Title",
            "Episode Count",
        ]);

        let mut shows = self.0.clone();

        shows.sort_by(|a, b| a.name.cmp(&b.name));

        for show in shows {
            table.add_row(vec![
                bs58::encode(show.id.to_be_bytes())
                    .into_string(),
                show.name.clone(),
                show.episode_count.to_string(),
            ]);
        }

        write!(f, "{table}")
    }
}

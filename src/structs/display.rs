use comfy_table::{presets::NOTHING, Table};
use ptree::print_tree;
use tallymarks::tally_marks_spaced;
use xvii::Roman;

use crate::cli::Numeral;

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
                show.year
                    .map(|y| y.to_string())
                    .unwrap_or("".to_string()),
                show.rating
                    .map(|r| r.to_string())
                    .unwrap_or("".to_string()),
            ]);
        }

        write!(f, "{table}")
    }
}

#[derive(Clone)]
pub struct TrackedShows {
    pub shows: Vec<TrackedShow>,
    pub id: bool,
    pub tree: bool,
    pub numeral: Numeral,
}

impl std::fmt::Display for TrackedShows {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if self.tree {
            print_tree(self).unwrap();
            return Ok(());
        }

        let num_str = match self.numeral {
            Numeral::Arabic => |x: usize| x.to_string(),
            Numeral::Roman => |x: usize| {
                Roman::new(x as u16).unwrap().to_string()
            },
            Numeral::TallyMarks => tally_marks_spaced,
        };

        let mut table = Table::new();

        table.load_preset(NOTHING);

        if self.id {
            table.set_header(vec![
                "ID", "Title", "Season", "Episode",
            ]);
        } else {
            table.set_header(vec![
                "Title", "Season", "Episode",
            ]);
        }

        let mut shows = self.shows.clone();

        shows.sort_by(|a, b| a.title.cmp(&b.title));

        for show in shows {
            table.add_row(if self.id {
                vec![
                    bs58::encode(show.id.to_be_bytes())
                        .into_string(),
                    show.title.clone(),
                    num_str(show.last_episode.0),
                    num_str(show.last_episode.1),
                ]
            } else {
                vec![
                    show.title.clone(),
                    num_str(show.last_episode.0),
                    num_str(show.last_episode.1),
                ]
            });
        }

        writeln!(f, "{table}")
    }
}

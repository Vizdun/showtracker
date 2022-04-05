use comfy_table::{presets::NOTHING, Table};
use ptree::print_tree;
use tallymarks::tally_marks_spaced;
use xvii::Roman;

use crate::cli::Numeral;

use super::{TrackedShow, Show};

pub const NUM_STR: fn(Numeral) -> fn(usize) -> String =
    |n| match n {
        Numeral::Arabic => |x: usize| x.to_string(),
        Numeral::Roman => |x: usize| {
            Roman::new(x as u16).unwrap().to_string()
        },
        Numeral::TallyMarks => tally_marks_spaced,
    };

pub struct Shows {
    pub shows: Vec<Show>,
    pub id: bool,
    pub stars: bool,
    pub numeral: Numeral,
}

impl std::fmt::Display for Shows {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let num_str = NUM_STR(self.numeral.clone());

        let rating_str = if self.stars {
            |x: f32| -> String {
                "*".repeat((x / 10.0 * 5.0) as usize)
            }
        } else {
            |x: f32| -> String { format!("{}", x) }
        };

        let mut table = Table::new();

        table.load_preset(NOTHING);

        if self.id {
            table.set_header(vec![
                "ID", "Title", "Year", "Rating",
            ]);
        } else {
            table.set_header(vec![
                "Title", "Year", "Rating",
            ]);
        }

        for show in &self.shows {
            table.add_row(if self.id {
                vec![
                    bs58::encode(show.id.to_be_bytes())
                        .into_string(),
                    show.title.clone(),
                    show.year
                        .map(|y| num_str(y as usize))
                        .unwrap_or("".to_string()),
                    show.rating
                        .map(|r| rating_str(r))
                        .unwrap_or("".to_string()),
                ]
            } else {
                vec![
                    show.title.clone(),
                    show.year
                        .map(|y| num_str(y as usize))
                        .unwrap_or("".to_string()),
                    show.rating
                        .map(|r| rating_str(r))
                        .unwrap_or("".to_string()),
                ]
            });
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

        let num_str = NUM_STR(self.numeral.clone());

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

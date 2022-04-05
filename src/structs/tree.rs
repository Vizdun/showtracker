use std::borrow::Cow;

use ptree::{TreeItem, Style, item::StringItem};

use super::{Episode, TrackedShow, display::TrackedShows};

impl TreeItem for Episode {
    type Child = StringItem;
    fn write_self<W: std::io::Write>(
        &self,
        f: &mut W,
        style: &Style,
    ) -> std::io::Result<()> {
        write!(f, "{}", style.paint(self.title.clone()))
    }
    fn children(&self) -> Cow<[Self::Child]> {
        Cow::from(vec![StringItem {
            text: self
                .airdate
                .format("%d. %m. %Y")
                .to_string(),
            children: vec![],
        }])
    }
}

#[derive(Clone)]
pub struct Season(usize, Vec<Episode>);

impl TreeItem for Season {
    type Child = Episode;
    fn write_self<W: std::io::Write>(
        &self,
        f: &mut W,
        style: &Style,
    ) -> std::io::Result<()> {
        write!(f, "Season {}", style.paint(self.0))
    }
    fn children(&self) -> Cow<[Self::Child]> {
        Cow::from(self.1.clone())
    }
}

impl TreeItem for TrackedShow {
    type Child = Season;
    fn write_self<W: std::io::Write>(
        &self,
        f: &mut W,
        style: &Style,
    ) -> std::io::Result<()> {
        write!(f, "{}", style.paint(self.title.clone()))
    }
    fn children(&self) -> Cow<[Self::Child]> {
        Cow::from(
            self.seasons
                .iter()
                .enumerate()
                .map(|(n, x)| Season(n + 1, x.clone()))
                .collect::<Vec<Season>>(),
        )
    }
}

impl TreeItem for TrackedShows {
    type Child = TrackedShow;
    fn write_self<W: std::io::Write>(
        &self,
        f: &mut W,
        style: &Style,
    ) -> std::io::Result<()> {
        write!(f, "{}", style.paint("Tracked Shows"))
    }
    fn children(&self) -> Cow<[Self::Child]> {
        Cow::from(self.shows.clone())
    }
}

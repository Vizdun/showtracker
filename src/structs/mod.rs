pub mod display;
pub mod tree;

use chrono::{Utc, DateTime};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Show {
    pub id: u32,
    pub title: String,
    pub year: Option<u16>,
    pub rating: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    pub title: String,
    pub airdate: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackedShow {
    pub id: u32,
    pub title: String,
    pub last_episode: (usize, usize),
    pub seasons: Vec<Vec<Episode>>,
}

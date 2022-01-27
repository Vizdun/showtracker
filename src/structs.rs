#[derive(Debug)]

pub struct Show {
    pub id:   u32,
    pub name: String
}

#[derive(Debug, Clone)]

pub struct TrackedShow {
    pub id:            u32,
    pub episode_count: u16,
    pub name:          String
}

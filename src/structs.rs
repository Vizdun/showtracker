#[derive(Debug)]

pub struct Show {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Clone)]

pub struct TrackedShow {
    pub id: u32,
    pub episode_count: u16,
    pub name: String,
}

#[derive(Clone)]

pub struct ShowPrintable {
    pub id: u32,
    pub name: String,
}

pub struct ShowsPrintable {
    pub shows: Vec<ShowPrintable>,
}

impl std::fmt::Display for ShowsPrintable {
    fn fmt(
        &self,
        fmt: &mut std::fmt::Formatter,
    ) -> std::result::Result<(), std::fmt::Error> {
        if self.shows.is_empty() {
            return Ok(());
        }
        let mut temp_arr = self.shows.clone();
        temp_arr.sort_by(|a, b| {
            a.name.len().partial_cmp(&b.name.len()).unwrap()
        });
        let longest_name = temp_arr.last().unwrap();

        writeln!(
            fmt,
            "|{: <6}|{: <longest$}|",
            "ID",
            "Title",
            longest = longest_name.name.len()
        )
        .unwrap();

        writeln!(
            fmt,
            "|{:-<6}|{:-<longest$}|",
            "-",
            "-",
            longest = longest_name.name.len()
        )
        .unwrap();

        for (indx, show) in (&self.shows).iter().enumerate()
        {
            write!(
                fmt,
                "|{:0>6}|{: <longest$}|",
                bs58::encode(show.id.to_le_bytes())
                    .into_string(),
                show.name,
                longest = longest_name.name.len()
            )
            .unwrap();
            if indx != self.shows.len() - 1 {
                writeln!(fmt).unwrap();
            }
        }
        Ok(())
    }
}

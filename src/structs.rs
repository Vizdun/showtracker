#[derive(Debug, serde::Serialize, serde::Deserialize)]

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

#[derive(Clone)]

pub struct ShowPrintable {
    pub id: u32,
    pub name: String,
    pub year: u8,
}

pub struct ShowsPrintable {
    pub shows: Vec<ShowPrintable>,
    pub years: bool,
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
            "|{: <6}|{: <longest$}|{}",
            "ID",
            "Title",
            if self.years {
                String::from("Year|")
            } else {
                String::from("")
            },
            longest = longest_name.name.len()
        )
        .unwrap();

        writeln!(
            fmt,
            "|{:-<6}|{:-<longest$}|{}",
            "-",
            "-",
            if self.years {
                format!("{:-<4}|", "-")
            } else {
                String::from("")
            },
            longest = longest_name.name.len()
        )
        .unwrap();

        for (indx, show) in (&self.shows).iter().enumerate()
        {
            write!(
                fmt,
                "|{:0>6}|{: <longest$}|{}",
                bs58::encode(show.id.to_le_bytes())
                    .into_string(),
                show.name,
                if self.years {
                    format!(
                        "{}|",
                        if show.year == 0 {
                            String::from("????")
                        } else {
                            (1880 + (show.year as u32))
                                .to_string()
                        },
                    )
                } else {
                    String::from("")
                },
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

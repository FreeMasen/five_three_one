use askama::Template;
use serde::Serialize;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
pub struct IndexArgs {
    #[structopt(short, long)]
    pub path: PathBuf,
}

#[derive(Debug, Clone, Template)]
#[template(path = "index.html")]
struct Index {
    pub years: Vec<Year>,
}

impl Index {
    pub fn add_month(&mut self, m: Month) {
        for year in &mut self.years {
            if year.year == m.year {
                year.months.push(m);
                return;
            }
        }
        self.years.push(Year { year: m.year, months: vec![m] });
    }
    pub fn sort(&mut self) {
        for year in &mut self.years {
            year.sort();
        }
        self.years.sort_by(|a, b| b.year.cmp(&a.year));
    }
}

#[derive(Debug, Clone, Serialize)]
struct Year {
    pub year: u64,
    pub months: Vec<Month>
}

impl Year {
    pub fn sort(&mut self) {
        self.months.sort_by(|a, b| a.month.cmp(&b.month));
    }
}

#[derive(Debug, Clone, Serialize)]
struct Month {
    name: String,
    year: u64,
    month: u8,
    href: String,
}

type R<T> = Result<T, Box<dyn std::error::Error>>;

pub fn run_index(args: IndexArgs) -> R<()> {
    let mut index = Index { years: Vec::new() };
    for ent in std::fs::read_dir(&args.path)? {
        if let Ok(ent) = ent {
            if let Some(month) = path_to_month(&ent.path()) {
                index.add_month(month);
            }
        }
    }
    index.sort();
    let html = index.render()?;
    std::fs::write(args.path.join("index.html"), html)?;
    Ok(())
}

fn path_to_month(p: &PathBuf) -> Option<Month> {
    const MONTHS: &'static [&'static str; 12] = &[
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let stem = p.file_stem()?.to_str()?;
    let mut parts = stem.split('-');
    let year = parts.next()?;
    let month = parts.next()?;
    let year: u64 = year.parse().ok()?;
    let month: u8 = month.parse().ok()?;
    let name = MONTHS[month.saturating_sub(1) as usize].to_string();
    let href = p.file_name()?.to_str()?.to_string();
    Some(Month {
        name,
        year,
        month,
        href,
    })
}

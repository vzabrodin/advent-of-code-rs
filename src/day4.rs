use anyhow::{anyhow, Error, Ok, Result};
use std::{fs, str::FromStr};

#[derive(Debug, Clone, Copy)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn includes(&self, other: Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: Range) -> bool {
        self.start <= other.start && self.end >= other.start
    }
}

impl FromStr for Range {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (start_str, end_str) = match s.split_once('-') {
            Some((a, b)) => (a, b),
            _ => return Err(anyhow!("Invalid format")),
        };

        let start = start_str.parse::<usize>()?;
        let end = end_str.parse::<usize>()?;

        Ok(Self { start, end })
    }
}

#[derive(Debug)]
struct RangePair {
    value1: Range,
    value2: Range,
}

impl FromStr for RangePair {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (value1_str, value2_str) = match s.split_once(',') {
            Some((a, b)) => (a, b),
            _ => return Err(anyhow!("Invalid format")),
        };

        let value1 = value1_str.parse::<Range>()?;
        let value2 = value2_str.parse::<Range>()?;

        Ok(Self { value1, value2 })
    }
}

fn main() -> Result<()> {
    let range_pairs = fs::read_to_string("src/day4.input")?
        .lines()
        .flat_map(|x| x.parse::<RangePair>())
        .collect::<Vec<RangePair>>();

    let include_pairs_count = range_pairs
        .iter()
        .filter(|x| x.value1.includes(x.value2) || x.value2.includes(x.value1))
        .count();

    let overlap_pairs_count = range_pairs
        .iter()
        .filter(|x| x.value1.overlaps(x.value2) || x.value2.overlaps(x.value1))
        .count();

    println!("{:#?}", include_pairs_count);
    println!("{:#?}", overlap_pairs_count);

    Ok(())
}

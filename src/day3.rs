use anyhow::{Error, Ok, Result};
use std::{fs, str::FromStr};

#[derive(Debug, Clone)]
struct Rucksack {
    priorities: Vec<u8>,
}

impl FromStr for Rucksack {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let priorities: Vec<u8> = s.chars().map(|x: char| to_priority(x)).collect();
        return Ok(Self { priorities });
    }
}

impl Rucksack {
    pub fn to_error_item_priority(&self) -> u32 {
        const COMPARTMENTS: usize = 2;

        let chunk_size = self.priorities.len() / COMPARTMENTS;
        let compartments = self.priorities.chunks(chunk_size);

        let items_bit_map = compartments
            .map(|x| x.iter().fold(0u64, |a, i| a | 1 << i))
            .fold(u64::MAX, |a, i| a & i);

        let duplicate_item_position = items_bit_map.trailing_zeros() + 1;

        duplicate_item_position
    }
}

#[derive(Debug)]
struct RucksackGroup {
    rucksacks: Vec<Rucksack>,
}

impl RucksackGroup {
    fn to_badge_priority(&self) -> u32 {
        let items_bit_map = self
            .rucksacks
            .iter()
            .map(|x| x.priorities.iter().fold(0u64, |a, i| a | 1 << i))
            .fold(u64::MAX, |a, i| a & i);

        let duplicate_item_position = items_bit_map.trailing_zeros() + 1;

        duplicate_item_position
    }
}

fn to_priority(c: char) -> u8 {
    const LOWER_START: u8 = b'a';
    const UPPER_START: u8 = b'A';
    const ALPHABET_LENGTH: u8 = 26;

    let u = c as u8;
    if u > LOWER_START {
        u - LOWER_START
    } else {
        u - UPPER_START + ALPHABET_LENGTH
    }
}

fn main() -> Result<()> {
    const CHUNK_SIZE: usize = 3;

    let rucksacks = fs::read_to_string("src/day3.input")?
        .lines()
        .flat_map(|x| x.parse::<Rucksack>())
        .collect::<Vec<Rucksack>>();

    let errors_sum: u32 = rucksacks
        .iter()
        .map(|rucksack| rucksack.to_error_item_priority())
        .sum();

    println!("{}", errors_sum);

    let badge_priority_sum: u32 = rucksacks
        .chunks(CHUNK_SIZE)
        .map(|chunk| RucksackGroup {
            rucksacks: chunk.to_vec(),
        })
        .map(|x| x.to_badge_priority())
        .sum();

    println!("{}", badge_priority_sum);

    Ok(())
}

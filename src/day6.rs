use anyhow::{Ok, Result};
use std::fs;

#[allow(dead_code)]
fn alphabet_map_to_string(alphabet_map: u32) -> String {
    const ALPHABET_SIZE: u8 = 26;

    (0..ALPHABET_SIZE)
        .map(|i| {
            if (alphabet_map >> i) & 1 == 1 {
                (i + b'a') as char
            } else {
                '_'
            }
        })
        .collect::<String>()
}

fn find_distinct_marker(input: &str, marker_size: usize) -> Option<usize> {
    input
        .as_bytes()
        .windows(marker_size)
        .map(|x| x.iter().fold(0u32, |a, x| a ^ 1 << (x - b'a')))
        .position(|x| x.count_ones() as usize == marker_size)
        .and_then(|x| Some(x + marker_size))
}

pub fn main() -> Result<()> {
    let input = fs::read_to_string("src/day6.input")?;

    match find_distinct_marker(&input, 4) {
        Some(position) => println!("{}", position),
        None => println!("position not found"),
    }

    match find_distinct_marker(&input, 14) {
        Some(position) => println!("{}", position),
        None => println!("position not found"),
    }

    Ok(())
}

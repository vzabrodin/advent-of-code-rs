use anyhow::anyhow;
use std::fs;

const DIGITS_1: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

const DIGITS_2: [&str; 20] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2",
    "3", "4", "5", "6", "7", "8", "9",
];

fn get_calibration_number(input: &str, digits: &[&str]) -> Option<usize> {
    let (left_index, _) = digits
        .iter()
        .enumerate()
        .filter_map(|(i, x)| Some((i, input.find(*x)?)))
        .min_by_key(|(_, x)| *x)?;

    let (right_index, _) = digits
        .iter()
        .enumerate()
        .filter_map(|(i, y)| Some((i, input.rfind(*y)?)))
        .max_by_key(|(_, x)| *x)?;

    Some(left_index % 10 * 10 + right_index % 10)
}

fn get_calibration_sum(input: &str, digits: &[&str]) -> Option<usize> {
    input
        .lines()
        .map(|x| get_calibration_number(x, digits))
        .sum()
}

pub fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("src/day1.input")?;

    let part1 = get_calibration_sum(input.as_str(), DIGITS_1.as_slice())
        .ok_or_else(|| anyhow!("Invalid format"))?;

    let part2 = get_calibration_sum(input.as_str(), DIGITS_2.as_slice())
        .ok_or_else(|| anyhow!("Invalid format"))?;

    println!("{}", part1);
    println!("{}", part2);

    Ok(())
}

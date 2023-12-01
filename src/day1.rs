use std::fs;

use anyhow::anyhow;

const DIGITS_1: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

const DIGITS_2: [&str; 20] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2",
    "3", "4", "5", "6", "7", "8", "9",
];

fn get_calibration_sum(input: &str, digits: &[&str]) -> Option<usize> {
    let calibration_values: Vec<_> = input
        .lines()
        .map(|x| {
            let (left_index, _) = digits
                .iter()
                .enumerate()
                .map(|(i, y)| Some((i, x.find(*y)?)))
                .filter_map(|x| x)
                .min_by_key(|(_, x)| *x)?;

            let (right_index, _) = digits
                .iter()
                .enumerate()
                .map(|(i, y)| Some((i, x.rfind(*y)?)))
                .filter_map(|x| x)
                .max_by_key(|(_, x)| *x)?;

            Some((left_index % 10, right_index % 10))
        })
        .map(|x| Some(x?.0 * 10 + x?.1))
        .collect::<Option<Vec<_>>>()?;

    Some(calibration_values.iter().sum())
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

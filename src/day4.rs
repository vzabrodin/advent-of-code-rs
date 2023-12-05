use anyhow::{anyhow, Ok};
use std::fs;

fn sum_winning_scores(input: &str) -> Result<u32, anyhow::Error> {
    input
        .lines()
        .map(|line| {
            let (_, card) = line
                .split_once(": ")
                .ok_or_else(|| anyhow!("Invalid format"))?;

            let (winning_numbers, numbers) = card
                .split_once(" | ")
                .ok_or_else(|| anyhow!("Invalid format"))?;

            let winning_numbers = winning_numbers
                .split(' ')
                .filter(|x| x.len() > 0)
                .map(|x| x.parse::<usize>().ok())
                .try_fold(0u128, |acc, x| Some(acc | (1 << x?)))
                .ok_or_else(|| anyhow!("Invalid format"))?;

            let numbers = numbers
                .split(' ')
                .filter(|x| x.len() > 0)
                .map(|x| x.parse::<usize>().ok())
                .try_fold(0u128, |acc, x| Some(acc | (1 << x?)))
                .ok_or_else(|| anyhow!("Invalid format"))?;

            let winning_numbers_count = (numbers & winning_numbers).count_ones();
            let score = if winning_numbers_count > 0 {
                1 << winning_numbers_count.saturating_sub(1)
            } else {
                0
            };

            Ok(score)
        })
        .sum::<Result<u32, _>>()
}

fn sum_winning_cards(input: &str) -> Result<u32, anyhow::Error> {
    let winning_cards = winning_cards(input)?;
    sum_winning_cards_rec(&winning_cards, winning_cards.len()).map(|x| x + winning_cards.len() as u32)
}

fn sum_winning_cards_rec(cards: &[u32], count: usize) -> Result<u32, anyhow::Error> {
    let mut sum = 0;

    for index in 0..count {
        let winning_numbers_count = cards[index];

        if winning_numbers_count > 0 {
            sum += winning_numbers_count;

            let slice = &cards[index + 1..];
            sum += sum_winning_cards_rec(&slice, winning_numbers_count as usize)?;
        }
    }

    Ok(sum)
}

#[allow(dead_code)]
fn winning_cards(input: &str) -> Result<Vec<u32>, anyhow::Error> {
    input
        .lines()
        .map(|line| {
            let (_, card) = line
                .split_once(": ")
                .ok_or_else(|| anyhow!("Invalid format"))?;

            let (winning_numbers, numbers) = card
                .split_once(" | ")
                .ok_or_else(|| anyhow!("Invalid format"))?;

            let winning_numbers = winning_numbers
                .split(' ')
                .filter(|x| x.len() > 0)
                .map(|x| x.parse::<usize>().ok())
                .try_fold(0u128, |acc, x| Some(acc | (1 << x?)))
                .ok_or_else(|| anyhow!("Invalid format"))?;

            let numbers = numbers
                .split(' ')
                .filter(|x| x.len() > 0)
                .map(|x| x.parse::<usize>().ok())
                .try_fold(0u128, |acc, x| Some(acc | (1 << x?)))
                .ok_or_else(|| anyhow!("Invalid format"))?;

            let winning_numbers_count = (numbers & winning_numbers).count_ones();
            Ok(winning_numbers_count)
        })
        .collect::<Result<Vec<u32>, _>>()
}

pub fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("src/day4.input")?;

    println!("{}", sum_winning_scores(&input)?);
    println!("{}", sum_winning_cards(&input)?);

    Ok(())
}

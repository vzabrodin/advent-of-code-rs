use std::{fs, str::FromStr};

#[derive(Debug)]
struct Score1 {
    value: usize,
}

#[derive(Debug)]
struct Score2 {
    value: usize,
}

/* Part 1 */
impl FromStr for Score1 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = match s.split_once(" ") {
            Some((a, b)) => (a, b),
            None => return Err(anyhow::anyhow!("Invalid input")),
        };

        let opponent_move = match a {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => return Err(anyhow::anyhow!("Invalid input")),
        };

        let response = match b {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => return Err(anyhow::anyhow!("Invalid input")),
        };

        let win_response = opponent_move % 3 + 1;

        let result = if response == win_response {
            6
        } else if response == opponent_move {
            3
        } else {
            0
        };

        return Ok(Self {
            value: result + response,
        });
    }
}

/* Part 2 */
impl FromStr for Score2 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = match s.split_once(" ") {
            Some((a, b)) => (a, b),
            None => return Err(anyhow::anyhow!("Invalid input")),
        };

        let opponent_move = match a {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => return Err(anyhow::anyhow!("Invalid input")),
        };

        let result = match b {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => return Err(anyhow::anyhow!("Invalid input")),
        };

        let response = match b {
            "X" => (opponent_move + 1) % 3 + 1,
            "Y" => opponent_move,
            "Z" => opponent_move % 3 + 1,
            _ => return Err(anyhow::anyhow!("Invalid input")),
        };

        return Ok(Self {
            value: result + response,
        });
    }
}

pub fn main() -> std::io::Result<()> {
    let total_score_1: usize = fs::read_to_string("src/day2.input")?
        .lines()
        .flat_map(|x| x.parse::<Score1>())
        .map(|x| x.value)
        .sum();

    let total_score_2: usize = fs::read_to_string("src/day2.input")?
        .lines()
        .flat_map(|x| x.parse::<Score2>())
        .map(|x| x.value)
        .sum();

    println!("{}", total_score_1);
    println!("{}", total_score_2);

    Ok(())
}

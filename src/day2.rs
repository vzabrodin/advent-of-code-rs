use anyhow::{anyhow, Ok};
use std::{
    fs,
    ops::{Add, AddAssign},
    str::FromStr,
};

#[derive(Debug, Clone, Copy)]
struct RGB {
    red: usize,
    green: usize,
    blue: usize,
}

const MAX_RGB: RGB = RGB {
    red: 12,
    green: 13,
    blue: 14,
};

impl RGB {
    fn empty() -> RGB {
        RGB {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn new(red: usize, green: usize, blue: usize) -> RGB {
        RGB { red, green, blue }
    }

    fn is_possible(&self) -> bool {
        self.red <= MAX_RGB.red && self.green <= MAX_RGB.green && self.blue <= MAX_RGB.blue
    }

    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

impl Add for RGB {
    type Output = RGB;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(
            self.red + rhs.red,
            self.green + rhs.green,
            self.blue + rhs.blue,
        )
    }
}

impl AddAssign for RGB {
    fn add_assign(&mut self, rhs: Self) {
        self.red += rhs.red;
        self.green += rhs.green;
        self.blue += rhs.blue;
    }
}

impl FromStr for RGB {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rgb = s
            .split(',')
            .map(|x| x.trim_start().trim_end())
            .map(|x| match x.split_once(' ')? {
                (x, "red") => Some(Self::new(x.parse::<usize>().ok()?, 0, 0)),
                (x, "green") => Some(Self::new(0, x.parse::<usize>().ok()?, 0)),
                (x, "blue") => Some(Self::new(0, 0, x.parse::<usize>().ok()?)),
                _ => None,
            })
            .try_fold(RGB::empty(), |acc, x| Some(acc + x?))
            .ok_or_else(|| anyhow!("Invalid format"))?;

        Ok(rgb)
    }
}

struct Game {
    id: usize,
    sets: Vec<RGB>,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, game) = s.split_once(':').ok_or_else(|| anyhow!("Invalid format"))?;
        let id = id
            .split_once(' ')
            .and_then(|(_, id)| id.parse::<usize>().ok())
            .ok_or_else(|| anyhow!("Invalid format"))?;

        let sets: Result<Vec<RGB>, _> = game.split(';').map(|x| x.parse::<RGB>()).collect();

        Ok(Self { id, sets: sets? })
    }
}

impl Game {
    fn is_possible(&self) -> bool {
        self.sets.iter().all(|x| x.is_possible())
    }

    fn max_possible_set(&self) -> RGB {
        self.sets.iter().fold(RGB::empty(), |result, x| {
            RGB::new(
                result.red.max(x.red),
                result.green.max(x.green),
                result.blue.max(x.blue),
            )
        })
    }
}

pub fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("src/day2.input")?;

    let games = input
        .lines()
        .map(|x| x.parse::<Game>())
        .collect::<Result<Vec<Game>, _>>()?;

    let possible_games_count: usize = games.iter().filter(|x| x.is_possible()).map(|x| x.id).sum();
    println!("{possible_games_count}");

    let max_possible_sets = games.iter().map(|x| x.max_possible_set());
    let sum_of_powers: usize = max_possible_sets.map(|x| x.power()).sum();
    println!("{sum_of_powers}");

    Ok(())
}

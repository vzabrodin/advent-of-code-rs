use anyhow::{anyhow, Error, Ok, Result};
use std::{fs, str::FromStr};

#[derive(Debug)]
struct Crane {
    value: Vec<Vec<char>>,
}

impl FromStr for Crane {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut crane = Vec::new();
        for line in s.lines().rev().skip(1) {
            let mut index = 0;
            let mut crate_index = 0;

            while index < line.len() {
                if crate_index >= crane.len() {
                    crane.push(Vec::new())
                }

                if line[index..].starts_with('[') {
                    let char = line.chars().nth(index + 1).unwrap();
                    crane[crate_index].push(char);
                }

                index += 4;
                crate_index += 1;
            }
        }

        Ok(Self { value: crane })
    }
}

impl Crane {
    pub fn move_crates(&mut self, ms: &[Move]) {
        for m in ms {
            self.move_crate(&m);
        }
    }

    pub fn move_crates_range(&mut self, ms: &[Move]) {
        for m in ms {
            self.move_crate_range(&m);
        }
    }

    pub fn move_crate(&mut self, m: &Move) {
        for _ in 0..m.count {
            match self.value[m.from].pop() {
                Some(c) => self.value[m.to].push(c),
                None => continue,
            }
        }
    }

    pub fn move_crate_range(&mut self, m: &Move) {
        let len = self.value[m.to].len();
        for _ in 0..m.count {
            match self.value[m.from].pop() {
                Some(c) => self.value[m.to].insert(len, c),
                None => continue,
            }
        }
    }

    pub fn get_top_crates(&self) -> String {
        self.value
            .iter()
            .map(|x| x.last().unwrap_or(&' '))
            .collect::<String>()
    }
}

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let vec = s
            .split_whitespace()
            .filter_map(|x| x.parse::<usize>().ok())
            .collect::<Vec<usize>>();

        let (count, from, to) = match vec[..] {
            [count, from, to] => (count, from, to),
            _ => return Err(anyhow!("Invalid format")),
        };

        Ok(Self {
            count,
            from: from - 1,
            to: to - 1,
        })
    }
}

#[derive(Debug)]
struct Moves {
    values: Vec<Move>,
}

impl FromStr for Moves {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let values = s
            .lines()
            .map(|x| x.parse::<Move>())
            .collect::<Result<Vec<Move>, Self::Err>>()?;

        Ok(Self { values })
    }
}

pub fn main() -> Result<()> {
    let content = fs::read_to_string("src/day5.input")?;

    let (crane, moves) = content.split_once("\n\n").expect("Invalid format");

    {
        let mut crane = crane.parse::<Crane>()?;
        let moves = moves.parse::<Moves>()?.values;

        crane.move_crates(&moves);

        println!("{}", crane.get_top_crates());
    }

    {
        let mut crane = crane.parse::<Crane>()?;
        let moves = moves.parse::<Moves>()?.values;

        crane.move_crates_range(&moves);

        println!("{}", crane.get_top_crates());
    }

    Ok(())
}

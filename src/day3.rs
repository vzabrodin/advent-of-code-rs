use crate::grid::Grid;
use anyhow::{anyhow, Ok};
use std::{fs, str::FromStr};

impl FromStr for Grid<char> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let columns = s
            .lines()
            .nth(0)
            .ok_or_else(|| anyhow!("Incorrect format"))?
            .len();

        let chars = s.lines().flat_map(|x| x.chars()).collect::<Vec<_>>();
        let rows = chars.len() / columns;
        Ok(Grid::new(chars, rows, columns))
    }
}

impl Grid<char> {
    fn part1(&self) -> usize {
        let mut sum = 0;
        for row in 0..self.rows() {
            let mut range: Option<(usize, usize)> = None;
            let mut has_adjacent_gears = false;
            let slice_row = self.slice_row(row);
            for column in 0..self.columns() {
                if slice_row[column].is_numeric() {
                    has_adjacent_gears |= self.has_adjacent_gears(row, column);
                    range = match range {
                        Some(r) => Some((r.0, column)),
                        None => Some((column, column)),
                    };
                } else {
                    if let Some(r) = range {
                        if has_adjacent_gears {
                            let number = slice_row[r.0..=r.1]
                                .iter()
                                .collect::<String>()
                                .parse::<usize>()
                                .unwrap();
                            sum += number;
                        }

                        range = None;
                        has_adjacent_gears = false;
                    }
                }
            }

            if let Some(r) = range {
                if has_adjacent_gears {
                    let number = self.slice_row(row)[r.0..=r.1]
                        .iter()
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap();
                    sum += number;
                }
            }
        }
        sum
    }

    fn has_adjacent_gears(&self, row: usize, column: usize) -> bool {
        let adjacent_points = [
            (row.saturating_sub(1), column.saturating_sub(1)),
            (row.saturating_sub(1), column),
            (row.saturating_sub(1), column + 1),
            (row, column.saturating_sub(1)),
            (row, column + 1),
            (row + 1, column.saturating_sub(1)),
            (row + 1, column),
            (row + 1, column + 1),
        ];

        adjacent_points
            .iter()
            .any(|(r, c)| match self.get(*r as usize, *c) {
                Some(cell) => !cell.is_numeric() && *cell != '.',
                None => false,
            })
    }
}

pub fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("src/day3.input")?;
    let grid: Grid<char> = input.parse()?;

    println!("{}", grid.part1());

    Ok(())
}

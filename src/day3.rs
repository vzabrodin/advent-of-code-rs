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

    fn part2(&self) -> usize {
        let mut sum = 0;
        for row in 0..self.rows() {
            let slice_row = self.slice_row(row);
            for column in 0..self.columns() {
                if slice_row[column] != '*' {
                    continue;
                }

                dbg!(self.count_adjacent_numbers(row, column));

                if let Some(adjacent_numbers) = self.adjacent_numbers(row, column) {
                    sum += adjacent_numbers.0 * adjacent_numbers.1;
                }
            }
        }
        sum
    }

    fn count_adjacent_numbers(&self, row: usize, column: usize) -> usize {
        let adjacent_points = [
            (row.checked_sub(1), column.checked_sub(1)),
            (row.checked_sub(1), Some(column)),
            (row.checked_sub(1), Some(column + 1)),
            (Some(row), column.checked_sub(1)),
            (Some(row), Some(column + 1)),
            (Some(row + 1), column.checked_sub(1)),
            (Some(row + 1), Some(column)),
            (Some(row + 1), Some(column + 1)),
        ]
        .map(|(row, column)| Some((row?, column?)));

        let adjacent_points = adjacent_points.iter().filter_map(|x| *x);

        adjacent_points
            .filter(|(r, c)| {
                self.get(*r as usize, *c)
                    .map(|cell| cell.is_numeric())
                    .unwrap_or(false)
            })
            .count()
    }

    fn adjacent_numbers(&self, row: usize, column: usize) -> Option<(usize, usize)> {
        let adjacent_points = [
            (row.checked_sub(1), column.checked_sub(1)),
            (row.checked_sub(1), Some(column)),
            (row.checked_sub(1), Some(column + 1)),
            (Some(row), column.checked_sub(1)),
            (Some(row), Some(column + 1)),
            (Some(row + 1), column.checked_sub(1)),
            (Some(row + 1), Some(column)),
            (Some(row + 1), Some(column + 1)),
        ]
        .map(|(row, column)| Some((row?, column?)));

        let adjacent_points = adjacent_points.iter().filter_map(|x| *x);

        let mut prev_number: Option<usize> = None;

        for (row, column) in adjacent_points {
            if self.is_numeric(row, column) {
                let slice = self.slice_row(row);
                let start = {
                    let mut position = column;
                    loop {
                        match slice.get(position) {
                            Some(x) if !x.is_numeric() => break position + 1,
                            Some(_) => (),
                            None => break position,
                        }

                        if position == 0 {
                            break position;
                        }

                        position = position.saturating_sub(1);
                    }
                };

                let end = {
                    let mut position = column;
                    loop {
                        match slice.get(position) {
                            Some(x) if !x.is_numeric() => break position,
                            Some(_) => (),
                            None => break position,
                        }
                        position += 1;
                    }
                };

                let number = &slice[start..end].iter().collect::<String>();
                dbg!(number);
                let number = number.parse::<usize>().unwrap();

                if let Some(prev_number) = prev_number {
                    return Some((prev_number, number));
                } else {
                    prev_number = Some(number);
                }
            }
        }

        None
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

    fn is_numeric(&self, row: usize, column: usize) -> bool {
        match self.get(row, column) {
            Some(cell) => cell.is_numeric(),
            _ => false,
        }
    }
}

pub fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("src/day3.input")?;
    let grid: Grid<char> = input.parse()?;

    println!("{}", grid.part1());
    println!("{}", grid.part2());

    Ok(())
}

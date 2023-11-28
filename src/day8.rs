use crate::grid::Grid;
use anyhow::{anyhow, Ok, Result};
use std::{char, collections::HashSet, fs, str::FromStr};

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
    pub fn count_visible_trees(&self) -> usize {
        let row_walkers = (0..self.rows()).flat_map(|row| {
            [
                Box::new(
                    self.row_iter(row)
                        .enumerate()
                        .map(move |(column, tree)| (row, column, tree)),
                ),
                Box::new(
                    self.row_iter(row)
                        .rev()
                        .enumerate()
                        .map(move |(column, tree)| (row, self.columns() - column - 1, tree)),
                ),
            ] as [Box<dyn Iterator<Item = (usize, usize, char)>>; 2]
        });

        let column_walkers = (0..self.columns()).flat_map(|column| {
            [
                Box::new(
                    self.column_iter(column)
                        .enumerate()
                        .map(move |(row, tree)| (row, column, tree)),
                ),
                Box::new(
                    self.column_iter(column)
                        .rev()
                        .enumerate()
                        .map(move |(row, tree)| (self.rows() - row - 1, column, tree)),
                ),
            ] as [Box<dyn Iterator<Item = (usize, usize, char)>>; 2]
        });

        let walkers = row_walkers.chain(column_walkers);

        let mut visible_trees = HashSet::new();
        for walker in walkers {
            let mut tallest_tree = 0 as char;
            for (row, column, tree) in walker {
                if tree > tallest_tree {
                    visible_trees.insert((row, column));
                    tallest_tree = tree;
                }
            }
        }

        visible_trees.len()
    }

    pub fn max_score(&self) -> usize {
        (1..self.rows() - 1)
            .flat_map(|r| (1..self.columns() - 1).map(move |c| self.score(r, c)))
            .max()
            .unwrap_or(0)
    }

    fn score(&self, row: usize, column: usize) -> usize {
        let tree = self.get(row, column).unwrap();
        let left_walker = self.row_iter(row).rev().skip(self.columns() - column);
        let right_walker = self.row_iter(row).skip(column + 1);
        let up_walker = self.column_iter(column).rev().skip(self.rows() - row);
        let down_walker = self.column_iter(column).skip(row + 1);

        let walkers: [Box<dyn Iterator<Item = char>>; 4] = [
            Box::new(left_walker),
            Box::new(right_walker),
            Box::new(up_walker),
            Box::new(down_walker),
        ];

        let score = walkers
            .map(|walker| {
                let mut count = 0;
                for x in walker {
                    count += 1;
                    if &x >= tree {
                        break;
                    }
                }
                count
            })
            .iter()
            .product();

        score
    }
}

pub fn main() -> Result<()> {
    let input = fs::read_to_string("src/day8.input")?;
    let grid: Grid<char> = input.parse()?;

    println!("{:?}", grid.count_visible_trees());
    println!("{:?}", grid.max_score());

    Ok(())
}

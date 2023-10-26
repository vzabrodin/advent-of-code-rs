use anyhow::{anyhow, Ok, Result};
use std::fs;

fn to_sizes(s: &str) -> Result<Vec<usize>> {
    let mut lines = s.lines().peekable();

    let mut pwd = Vec::new();
    let mut sizes = Vec::new();

    while let Some(line) = lines.next() {
        match line.rsplit_once(' ') {
            Some(("$ cd", "..")) => {
                let size = pwd.pop().ok_or(anyhow!("Cannot pop an ampty stack"))?;
                *pwd.last_mut().ok_or(anyhow!("Cannot pop an ampty stack"))? += size;
                sizes.push(size);
            }
            Some(("$ cd", _)) => {
                pwd.push(0);
            }
            Some(("$", "ls")) => {
                let mut size = 0;
                while let Some(line) = lines.next_if(|x| !x.starts_with('$')) {
                    size += match line.rsplit_once(' ') {
                        Some((size, _)) => size.parse::<usize>().unwrap_or(0),
                        None => 0,
                    };
                }

                if let Some(s) = pwd.last_mut() {
                    *s += size;
                }
            }
            _ => return Err(anyhow!("Invalid format")),
        }
    }

    while let Some(size) = pwd.pop() {
        if let Some(last) = pwd.last_mut() {
            *last += size;
        }
        sizes.push(size);
    }

    sizes.sort();
    Ok(sizes)
}

fn get_result_1(sizes: &Vec<usize>) -> usize {
    sizes.iter().filter(|x| x <= &&100000).sum::<usize>()
}

fn get_result_2(sizes: &Vec<usize>) -> Option<usize> {
    const TOTAL_SPACE: usize = 70000000;
    const ENOUGH_SPACE: usize = 30000000;
    let used_space = sizes.last()?;
    let unused_space = TOTAL_SPACE - used_space;
    let space_to_free = ENOUGH_SPACE - unused_space;

    sizes.iter().find(|x| x >= &&space_to_free).map(|x| *x)
}

pub fn main() -> Result<()> {
    let input = fs::read_to_string("src/day7.input")?;
    let sizes = to_sizes(&input)?;

    println!("{}", get_result_1(&sizes));
    println!("{}", get_result_2(&sizes).expect("must work"));

    Ok(())
}

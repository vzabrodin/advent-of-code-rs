use std::{fs, io};

pub fn main() -> io::Result<()> {
    let mut sums = fs::read_to_string("src/day1.input")?
        .lines()
        .collect::<Vec<&str>>()
        .split(|x| x.is_empty())
        .map(|chunk| chunk.iter().flat_map(|x| x.parse::<u32>()).sum())
        .collect::<Vec<u32>>();

    sums.sort_by(|a, b| b.cmp(a));

    //let result: u32 = sums.iter().take(1).sum();
    let result: u32 = sums.iter().take(3).sum();

    println!("{}", result);

    Ok(())
}

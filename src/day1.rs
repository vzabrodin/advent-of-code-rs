use std::{io, fs};

fn main() -> io::Result<()> {
    let content = fs::read_to_string("src/day1.input")?;

    let mut sums: Vec<u32> = content
        .split("\n\n")
        .map(|chunk| chunk.lines().flat_map(|x| x.parse::<u32>()).sum::<u32>())
        .collect();

    sums.sort_by(|a, b| b.cmp(a));

    //let result: u32 = sums.into_iter().take(1).sum();
    let result: u32 = sums.into_iter().take(3).sum();

    println!("{}", result);

    Ok(())
}

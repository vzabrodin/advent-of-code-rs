mod day1;
mod day2;
mod day3;
mod day4;
mod grid;

use anyhow::{Ok, Result};

fn main() -> Result<()> {
    println!("# Day 1");
    day1::main()?;
    println!();

    println!("# Day 2");
    day2::main()?;
    println!();

    println!("# Day 3");
    day3::main()?;
    println!();

    println!("# Day 4");
    day4::main()?;
    println!();

    Ok(())
}

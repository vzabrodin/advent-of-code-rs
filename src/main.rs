mod day1;
mod day2;

use anyhow::{Ok, Result};

fn main() -> Result<()> {
    println!("# Day 1");
    day1::main()?;
    println!();

    println!("# Day 2");
    day2::main()?;
    println!();

    Ok(())
}
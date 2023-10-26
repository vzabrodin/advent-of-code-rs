mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

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

    println!("# Day 5");
    day5::main()?;
    println!();

    println!("# Day 6");
    day6::main()?;
    println!();

    println!("# Day 7");
    day7::main()?;
    println!();

    Ok(())
}

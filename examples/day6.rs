use anyhow::Result;
use aoc2022::*;
use aoc2022::day6::*;

fn main() -> Result<()> {
    // Day 6
    let contents = read_day_data(6)?;

    // 6.1
    println!("Day 6, Answer 1: {}", first_distinct_subword(&contents, 4)?);

    // 6.2
    println!(
        "Day 6, Answer 2: {}",
        first_distinct_subword(&contents, 14)?
    );

    Ok(())
}

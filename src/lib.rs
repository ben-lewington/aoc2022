pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

use std::fs;

use anyhow::Result;

pub fn read_day_data(day: usize) -> Result<String> {
    let x = fs::read_to_string(format!("./data/day{}.txt", day))?;
    Ok(x)
}

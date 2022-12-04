use anyhow::Result;
use aoc2022::day4::*;
use aoc2022::*;

fn main() -> Result<()> {
    //Day 4
    let contents = read_day_data(4)?;
    let section_range_pair_parser = SectionRangePair::parser(',', '-');

    // 4.1
    let l = &contents
        .split("\r\n")
        .map(|s| section_range_pair_parser(s).unwrap())
        .filter(|srp| srp.is_subset())
        .count();

    println!("Day 4, Answer 1: {}", l);

    // 4.2
    let l = &contents
        .split("\r\n")
        .map(|s| section_range_pair_parser(s).unwrap())
        .filter(|srp| !srp.is_no_overlap())
        .count();

    println!("Day 4, Answer 2: {}", l);

    Ok(())
}

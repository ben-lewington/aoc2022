use aoc2022::read_day_data;

use anyhow::Result;

fn main() -> Result<()> {
    // Day 1
    let contents = read_day_data(1)?;

    // 1.1
    let c = contents
        .split("\n\n")
        .map(|b| {
            b.split("\n")
                .filter(|s| s.len() > 0)
                .map(|s| s.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .max()
        .unwrap();
    println!("Day 1, Answer 1: {:?}", c);

    // 1.2
    let mut c = contents
        .split("\n\n")
        .map(|b| {
            b.split("\n")
                .filter(|s| s.len() > 0)
                .map(|s| s.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect::<Vec<usize>>();
    c.sort_unstable();
    c.reverse();
    let v = c.iter().take(3).sum::<usize>();
    println!("Day 1, Answer 2: {}", v);
    Ok(())
}

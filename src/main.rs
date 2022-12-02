use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    // Day 1
    // let contents = fs::read_to_string("./day1.txt")?;

    // 1.1
    // let c = contents
    //     .split("\n\n")
    //     .map(|b| {
    //         b.split("\n")
    //             .filter(|s| s.len() > 0)
    //             .map(|s| s.parse::<usize>().unwrap())
    //             .sum::<usize>()
    //     })
    //     .max()
    //     .unwrap();
    // println!("{:?}", c);

    // 1.2
    // let mut c = contents
    //     .split("\n\n")
    //     .map(|b| {
    //         b.split("\n")
    //             .filter(|s| s.len() > 0)
    //             .map(|s| s.parse::<usize>().unwrap())
    //             .sum::<usize>()
    //     }).collect::<Vec<usize>>();
    // c.sort_unstable();
    // c.reverse();
    // let v = c.iter().take(3).sum::<usize>();
    // println!("{}", v);
    Ok(())
}

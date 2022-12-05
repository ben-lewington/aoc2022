use anyhow::Result;
use aoc2022::day5::*;
use aoc2022::*;

fn main() -> Result<()> {
    // Day 5
    let contents = read_day_data(5)?;

    // 5.1
    let mut cs = CrateStack::new();

    let _ = &contents
        .split("\r\n")
        .filter(|s| s.len() > 0)
        .map(|x| {
            let mut y = x.split(" ");
            y.next();
            let m = y.next().unwrap().parse::<usize>().unwrap();
            y.next();
            let f = y.next().unwrap().parse::<usize>().unwrap();
            y.next();
            let t = y.next().unwrap().parse::<usize>().unwrap();
            (m, f, t)
        })
        .for_each(|(multiplicity, index_from, index_to)| {
            cs.move_crates(multiplicity, index_from, index_to);
        });

    let cs =
        cs.0.iter_mut()
            .filter(|s| s.len() > 0)
            .map(|s| s.pop().unwrap())
            .collect::<String>();

    println!("Day 5, Answer 1: {:?}", cs);

    // 5.2

    let mut cs = CrateStack::new();

    let _ = &contents
        .split("\r\n")
        .filter(|s| s.len() > 0)
        .map(|x| {
            let mut y = x.split(" ");
            y.next();
            let m = y.next().unwrap().parse::<usize>().unwrap();
            y.next();
            let f = y.next().unwrap().parse::<usize>().unwrap();
            y.next();
            let t = y.next().unwrap().parse::<usize>().unwrap();
            (m, f, t)
        })
        .for_each(|(multiplicity, index_from, index_to)| {
            cs.move_crates_in_bulk(multiplicity, index_from, index_to);
        });

    let cs =
        cs.0.iter_mut()
            .filter(|s| s.len() > 0)
            .map(|s| s.pop().unwrap())
            .collect::<String>();

    println!("Day 5, Answer 2: {:?}", cs);

    Ok(())
}

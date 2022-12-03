use anyhow::{bail, Result};
use aoc2022::day3::*;
use aoc2022::*;

fn main() -> Result<()> {
    // Day 3
    let contents = read_day_data(3)?;

    // 3.1
    let c = &contents
        .split("\r\n")
        .filter(|x| x.len() % 2 == 0)
        .map(|x| {
            let (a, b) = x.split_at(x.len() / 2);
            shared_chars(a, b)
                .into_iter()
                .map(|c| char_score(c).unwrap())
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("Day 3, Answer 1: {}", c);

    let mut c = contents.split("\r\n");
    let mut p: Vec<usize> = vec![];
    while let Some(s1) = c.next() {
        let s2 = c.next().unwrap();
        let s3 = c.next().unwrap();
        let mut d: Vec<char> = vec![];
        for c in s1.chars() {
            if s2.contains(c) && s3.contains(c) && !d.contains(&c) {
                d.push(c);
            }
        }
        if d.len() != 1 {
            bail!("Too many common badges")
        }
        p.push(char_score(d.pop().unwrap()).unwrap());
    }

    println!("Day 3, Answer 2: {}", p.iter().sum::<usize>());
    Ok(())
}

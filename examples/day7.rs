use std::collections::HashMap;

use anyhow::{bail, Result};
use aoc2022::*;

#[derive(Debug, PartialEq, Eq, Clone)]
struct File {
    size: usize,
    name: String,
}

const PART_1_UPPER_BOUND: usize = 100_000;
const PART_2_TOTAL: usize = 70_000_000;
const PART_2_LOWER_BOUND: usize = 30_000_000;

fn main() -> Result<()> {
    // Day 7
    let contents = read_day_data(7)?;

    let mut iter = contents.split("\r\n").peekable();
    let mut cwd: String = "".to_string();
    let mut dirs: HashMap<String, Vec<File>> = HashMap::new();
    while let Some(l) = iter.next() {
        let mut s = l.split(' ');
        let Some(ss) = s.next() else { continue; };
        if ss != "$" {
            continue;
        }
        let Some(cmd) = s.next() else { bail!("something's wrong") };
        if cmd == "cd" {
            let Some(dir) = s.next() else { bail!("something's wrong") };
            if dir == "/" {
                cwd = "/".to_string();
            } else if dir == ".." {
                let x = cwd.clone();
                let mut y = x.rsplitn(2, "/");
                y.next();
                let x = y.next().unwrap();
                cwd = x.to_string();
            } else if dir == "." {
            } else {
                if cwd == "/" {
                    cwd = format!("/{}", dir);
                } else {
                    cwd = format!("{}/{}", cwd, dir);
                }
            }
            if dir == ".." {
                continue;
            }
            dirs.entry(cwd.clone()).or_insert(Vec::new());
        } else if cmd == "ls" {
            while let Some(&s) = iter.peek() {
                if s.starts_with("$") {
                    break;
                }
                let s = iter.next().expect("we peeked it");
                if s.starts_with("dir") {
                    continue;
                }
                let (size, name) = s.split_at(s.find(" ").expect("this is what it looks like"));
                dirs.entry(cwd.clone()).and_modify(|v| {
                    v.push(File {
                        size: size.parse().unwrap(),
                        name: name.trim_start().to_string(),
                    })
                });
            }
        } else {
            bail!("Unexpected command.")
        }
    }

    let mut sums: HashMap<String, usize> = HashMap::new();
    for outer in dirs.keys() {
        sums.insert(outer.clone(), 0);
        for (inner, files) in dirs.iter() {
            if inner.starts_with(outer) {
                sums.entry(outer.clone()).and_modify(|s| {
                    *s += files
                        .iter()
                        .map(|File { size, name: _ }| size)
                        .sum::<usize>()
                });
            }
        }
    }

    let c = sums
        .values()
        .filter(|&v| v <= &PART_1_UPPER_BOUND)
        .sum::<usize>();

    println!("Day 7, Answer 1: {}", c);

    let unused_space = PART_2_TOTAL - sums.get("/").unwrap();
    let mut candidates: HashMap<String, usize> = HashMap::new();
    for key in sums.keys().clone() {
        let si = sums.get(key).unwrap();
        let c = unused_space + si;
        if c > PART_2_LOWER_BOUND {
            candidates.insert(key.clone(), *si);
        }
    }

    let c = candidates.values().min().unwrap();

    println!("Day 7, Answer 1: {}", c);

    Ok(())
}

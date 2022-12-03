use anyhow::{bail, Result};

fn ascii(c: char) -> usize {
    c as usize
}

fn between(check: usize, lower: usize, upper: usize) -> bool {
    (check <= upper) && (check >= lower)
}

pub fn char_score(c: char) -> Result<usize> {
    let c = ascii(c);
    if between(c, ascii('a'), ascii('z')) {
        return Ok(c - ascii('a') + 1);
    }
    if between(c, ascii('A'), ascii('Z')) {
        return Ok(c - ascii('A') + 27);
    }
    bail!("{} is not an alphanumeric character", c)
}

pub fn shared_chars(a: &str, b: &str) -> Vec<char> {
    let mut v = vec![];
    for c in a.chars() {
        if b.contains(c) && !v.contains(&c) {
            v.push(c);
        }
    }
    v
}

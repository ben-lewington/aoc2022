use anyhow::{bail, Result};

pub fn first_distinct_subword(c: &str, l: usize) -> Result<usize> {
    for i in 1..=c.len() {
        if c.len() - i > l {
            let x = &c[i - 1..i + l - 1];
            let mut v = Vec::<char>::with_capacity(l);
            for c in x.chars() {
                if v.contains(&c) {
                    break;
                }
                v.push(c);
            }
            if v.len() == l {
                return Ok(i + l - 1);
            }
        }
    }
    bail!("didn't find anything")
}

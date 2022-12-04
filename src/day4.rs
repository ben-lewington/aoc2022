use anyhow::{anyhow, Result};

#[derive(Eq, PartialEq, Debug)]
pub struct SectionRange {
    pub lower_bound: usize,
    pub upper_bound: usize,
}

impl SectionRange {
    pub fn parser(delimiter: char) -> impl Fn(&str) -> Result<SectionRange> {
        move |s| {
            let mut v = s.split(delimiter);
            let Some(min) = v.next() else {
                return Err(anyhow!("no beginning value"));
            };
            let Some(max) = v.next() else {
                return Err(anyhow!("no end value"));
            };
            Ok(SectionRange {
                lower_bound: min.parse()?,
                upper_bound: max.parse()?,
            })
        }
    }
}

#[derive(Debug)]
pub struct SectionRangePair {
    pub left: SectionRange,
    pub right: SectionRange,
}

impl SectionRangePair {
    pub fn parser(
        pair_delim: char,
        range_delim: char,
    ) -> impl Fn(&str) -> Result<SectionRangePair> {
        move |s| {
            let mut p = s.split(pair_delim);
            let Some(left) = p.next() else {
                return Err(anyhow!("no beginning value"));
            };
            let Some(right) = p.next() else {
                return Err(anyhow!("no end value"));
            };
            let p = SectionRange::parser(range_delim);
            Ok(SectionRangePair {
                left: p(left)?,
                right: p(right)?,
            })
        }
    }

    pub fn is_no_overlap(&self) -> bool {
        self.left.upper_bound < self.right.lower_bound
            || self.right.upper_bound < self.left.lower_bound
    }

    pub fn is_subset(&self) -> bool {
        (self.left.lower_bound <= self.right.lower_bound
            && self.right.upper_bound <= self.left.upper_bound)
            || (self.right.lower_bound <= self.left.lower_bound
                && self.left.upper_bound <= self.right.upper_bound)
    }
}

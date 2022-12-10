use anyhow::Result;
use aoc2022::*;

fn main() -> Result<()> {
    // Day 8
    let contents = read_day_data(8)?;

    let grid_rows: Vec<Vec<usize>> = contents
        .split("\r\n")
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect();

    let (rc, cc) = (grid_rows.len(), grid_rows[0].len());

    let mut grid_cols: Vec<Vec<usize>> = vec![];

    (0..cc).for_each(|_| {
        grid_cols.push(vec![]);
    });

    contents.split("\r\n").for_each(|l| {
        for (i, c) in l.chars().enumerate() {
            grid_cols[i].push(c.to_digit(10).unwrap() as usize);
        }
    });

    // Answer 1

    let mut results: Vec<Vec<bool>> = vec![];

    (0..rc).for_each(|_| {
        results.push(vec![]);
    });

    for r in 0..rc {
        for c in 0..cc {
            let visible = if r == 0 || r == rc - 1 {
                true
            } else if c == 0 || c == cc - 1 {
                true
            } else {
                let &tree = &grid_rows[r][c];
                let g_row = &grid_rows[r];
                let (rl, rr) = g_row.split_at(c);
                let (&rl_max, &rr_max) = (
                    rl.iter().max().unwrap_or(&0),
                    rr.iter().skip(1).max().unwrap_or(&0),
                );
                let g_col = &grid_cols[c];
                let (cl, cr) = g_col.split_at(r);
                let (&cl_max, &cr_max) = (
                    cl.iter().max().unwrap_or(&0),
                    cr.iter().skip(1).max().unwrap_or(&0),
                );
                if &tree
                    > [&rl_max, &rr_max, &cl_max, &cr_max]
                        .iter()
                        .min()
                        .unwrap_or(&&0)
                {
                    true
                } else {
                    false
                }
            };
            results[r].push(visible);
        }
    }

    println!(
        "Day 8, Answer 1: {}",
        results
            .iter()
            .map(|v| { v.iter().filter(|&&b| b).count() })
            .sum::<usize>()
    );


    // Answer 2

    let mut scenic_score: Vec<Vec<usize>> = vec![];

    (0..rc).for_each(|_| {
        scenic_score.push(vec![]);
    });

    for r in 0..rc {
        for c in 0..cc {
            let score = if r == 0 || r == rc - 1 {
                0
            } else if c == 0 || c == cc - 1 {
                0
            } else {
                let &tree = &grid_rows[r][c];

                let g_row = &grid_rows[r];
                let (rl, rr) = g_row.split_at(c);
                let rl_score: usize = if r == 1 {
                    1
                } else {
                    let mut ret: usize = 0;
                    for (i, rlt) in rl.iter().rev().enumerate() {
                        ret = i + 1;
                        if rlt >= &tree {
                            break;
                        }
                    }
                    ret
                };

                let rr_score: usize = if r == rc - 1 {
                    1
                } else {
                    let mut ret: usize = 0;
                    for (i, rrt) in rr.iter().skip(1).enumerate() {
                        ret = i + 1;
                        if rrt >= &tree {
                            break;
                        }
                    }
                    ret
                };

                let g_col = &grid_cols[c];
                let (cl, cr) = g_col.split_at(r);
                let cl_score: usize = if c == 1 {
                    1
                } else {
                    let mut ret: usize = 0;
                    for (i, clt) in cl.iter().rev().enumerate() {
                        ret = i + 1;
                        if clt >= &tree {
                            break;
                        }
                    }
                    ret
                };

                let cr_score: usize = if c == cc - 1 {
                    1
                } else {
                    let mut ret: usize = 0;
                    for (i, crt) in cr.iter().skip(1).enumerate() {
                        ret = i + 1;
                        if crt >= &tree {
                            break;
                        }
                    }
                    ret
                };

                rl_score * rr_score * cl_score * cr_score
            };
            scenic_score[r].push(score);
        }
    }

    println!(
        "Day 8, Answer 2: {}",
        scenic_score
            .iter()
            .map(|r| r.iter().max().unwrap_or(&0))
            .max()
            .unwrap_or(&0)
    );

    Ok(())
}

use anyhow::Result;
use aoc2022::day2::*;
use aoc2022::read_day_data;

fn main() -> Result<()> {
    // Day 2
    let contents = read_day_data(2)?;

    // 2.1
    let opponent_parser = RpsMove::parser('A', 'B', 'C');
    let player_parser = RpsMove::parser('X', 'Y', 'Z');
    let c = &contents
        .split("\r\n")
        .filter(|x| x.len() == 3)
        .map(|x| {
            let x = x.chars().collect::<Vec<char>>();
            let opponent = opponent_parser(x[0]).unwrap();
            let player = player_parser(x[2]).unwrap();
            RpsScenario::with_move_pair(opponent, player).score()
        })
        .sum::<usize>();

    println!("Day 2, Answer 1: {:?}", c);

    // 2.2
    let outcome_parser = RpsOutcome::parser('X', 'Y', 'Z');
    let c = &contents
        .split("\r\n")
        .filter(|x| x.len() == 3)
        .map(|x| {
            let x = x.chars().collect::<Vec<char>>();
            let opponent = opponent_parser(x[0]).unwrap();
            let outcome = outcome_parser(x[2]).unwrap();
            RpsScenario::with_move_outcome(opponent, outcome).score()
        })
        .sum::<usize>();

    println!("Day 2, Answer 2: {:?}", c);
    Ok(())
}

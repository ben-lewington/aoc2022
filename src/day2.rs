use anyhow::{bail, Result};

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum RpsMove {
    Rock,
    Paper,
    Scissors,
}

impl RpsMove {
    pub fn parser(rock: char, paper: char, scissors: char) -> impl Fn(char) -> Result<RpsMove> {
        move |input| {
            return if input == rock {
                Ok(RpsMove::Rock)
            } else if input == paper {
                Ok(RpsMove::Paper)
            } else if input == scissors {
                Ok(RpsMove::Scissors)
            } else {
                bail!("not a valid input")
            };
        }
    }

    pub fn score(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum RpsOutcome {
    Win,
    Draw,
    Lose,
}

impl RpsOutcome {
    pub fn parser(lose: char, draw: char, win: char) -> impl Fn(char) -> Result<RpsOutcome> {
        move |input| {
            return if input == lose {
                Ok(RpsOutcome::Lose)
            } else if input == draw {
                Ok(RpsOutcome::Draw)
            } else if input == win {
                Ok(RpsOutcome::Win)
            } else {
                bail!("not a valid input")
            };
        }
    }

    pub fn score(&self) -> usize {
        match self {
            Self::Lose => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum RpsScenario {
    MovePair {
        opponent: RpsMove,
        player: RpsMove,
    },
    MoveOutcome {
        opponent: RpsMove,
        outcome: RpsOutcome,
    },
}

impl RpsScenario {
    pub fn with_move_pair(opponent: RpsMove, player: RpsMove) -> Self {
        Self::MovePair { opponent, player }
    }

    pub fn with_move_outcome(opponent: RpsMove, outcome: RpsOutcome) -> Self {
        Self::MoveOutcome { opponent, outcome }
    }

    pub fn resolve(&self) -> (RpsMove, RpsOutcome) {
        match self {
            &Self::MovePair { opponent, player } => {
                let outcome = match player {
                    RpsMove::Rock => match opponent {
                        RpsMove::Rock => RpsOutcome::Draw,
                        RpsMove::Paper => RpsOutcome::Lose,
                        RpsMove::Scissors => RpsOutcome::Win,
                    },
                    RpsMove::Paper => match opponent {
                        RpsMove::Rock => RpsOutcome::Win,
                        RpsMove::Paper => RpsOutcome::Draw,
                        RpsMove::Scissors => RpsOutcome::Lose,
                    },
                    RpsMove::Scissors => match opponent {
                        RpsMove::Rock => RpsOutcome::Lose,
                        RpsMove::Paper => RpsOutcome::Win,
                        RpsMove::Scissors => RpsOutcome::Draw,
                    },
                };
                (player, outcome)
            }
            &Self::MoveOutcome { opponent, outcome } => {
                let player = match outcome {
                    RpsOutcome::Win => match opponent {
                        RpsMove::Rock => RpsMove::Paper,
                        RpsMove::Paper => RpsMove::Scissors,
                        RpsMove::Scissors => RpsMove::Rock,
                    },
                    RpsOutcome::Draw => match opponent {
                        RpsMove::Rock => RpsMove::Rock,
                        RpsMove::Paper => RpsMove::Paper,
                        RpsMove::Scissors => RpsMove::Scissors,
                    },
                    RpsOutcome::Lose => match opponent {
                        RpsMove::Rock => RpsMove::Scissors,
                        RpsMove::Paper => RpsMove::Rock,
                        RpsMove::Scissors => RpsMove::Paper,
                    },
                };
                (player, outcome)
            }
        }
    }

    pub fn score(&self) -> usize {
        let (m, o) = self.resolve();
        m.score() + o.score()
    }
}

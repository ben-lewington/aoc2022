use anyhow::{bail, Result};

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum RpsSelection {
    Rock,
    Paper,
    Scissors,
}

impl RpsSelection {
    pub fn decoder(r: char, p: char, s: char) -> impl Fn(char) -> Result<RpsSelection> {
        move |x| {
            return if x == r {
                Ok(RpsSelection::Rock)
            } else if x == p {
                Ok(RpsSelection::Paper)
            } else if x == s {
                Ok(RpsSelection::Scissors)
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
    pub fn decoder(l: char, d: char, w: char) -> impl Fn(char) -> Result<RpsOutcome> {
        move |x| {
            return if x == l {
                Ok(RpsOutcome::Lose)
            } else if x == d {
                Ok(RpsOutcome::Draw)
            } else if x == w {
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
    Move {
        opponent: RpsSelection,
        player: RpsSelection,
    },
    Given {
        opponent: RpsSelection,
        outcome: RpsOutcome,
    },
}

impl RpsScenario {
    pub fn new_with_move(opponent: RpsSelection, player: RpsSelection) -> Self {
        Self::Move { opponent, player }
    }

    pub fn new_with_outcome(opponent: RpsSelection, outcome: RpsOutcome) -> Self {
        Self::Given { opponent, outcome }
    }

    pub fn resolve(&self) -> (RpsSelection, RpsOutcome) {
        match self {
            &Self::Move {
                opponent: o,
                player: p,
            } => {
                let outcome = match p {
                    RpsSelection::Rock => match o {
                        RpsSelection::Rock => RpsOutcome::Draw,
                        RpsSelection::Paper => RpsOutcome::Lose,
                        RpsSelection::Scissors => RpsOutcome::Win,
                    },
                    RpsSelection::Paper => match o {
                        RpsSelection::Rock => RpsOutcome::Win,
                        RpsSelection::Paper => RpsOutcome::Draw,
                        RpsSelection::Scissors => RpsOutcome::Lose,
                    },
                    RpsSelection::Scissors => match o {
                        RpsSelection::Rock => RpsOutcome::Lose,
                        RpsSelection::Paper => RpsOutcome::Win,
                        RpsSelection::Scissors => RpsOutcome::Draw,
                    },
                };
                (p, outcome)
            }
            &Self::Given {
                opponent: opp,
                outcome: out,
            } => {
                let player = match out {
                    RpsOutcome::Win => match opp {
                        RpsSelection::Rock => RpsSelection::Paper,
                        RpsSelection::Paper => RpsSelection::Scissors,
                        RpsSelection::Scissors => RpsSelection::Rock,
                    },
                    RpsOutcome::Draw => match opp {
                        RpsSelection::Rock => RpsSelection::Rock,
                        RpsSelection::Paper => RpsSelection::Paper,
                        RpsSelection::Scissors => RpsSelection::Scissors,
                    },
                    RpsOutcome::Lose => match opp {
                        RpsSelection::Rock => RpsSelection::Scissors,
                        RpsSelection::Paper => RpsSelection::Rock,
                        RpsSelection::Scissors => RpsSelection::Paper,
                    },
                };
                (player, out)
            }
        }
    }

    pub fn score(&self) -> usize {
        let (m, o) = self.resolve();
        m.score() + o.score()
    }
}

use std::str::FromStr;

#[derive(Debug)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Move {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "a" | "x" => Ok(Self::Rock),
            "b" | "y" => Ok(Self::Paper),
            "c" | "z" => Ok(Self::Scissors),
            other => eyre::bail!("unrecognised move: {other}"),
        }
    }
}

pub struct Game {
    pub opponent: Move,
    pub player: Move,
}

impl Game {
    pub fn score(&self) -> u64 {
        // start with the player move's value
        let mut result = match &self.player {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        };

        // now add whether the player won/drew/lost
        result += match (&self.player, &self.opponent) {
            // win
            (Move::Rock, Move::Scissors)
            | (Move::Paper, Move::Rock)
            | (Move::Scissors, Move::Paper) => 6,

            // draw
            (Move::Rock, Move::Rock)
            | (Move::Paper, Move::Paper)
            | (Move::Scissors, Move::Scissors) => 3,

            // loss
            (Move::Rock, Move::Paper)
            | (Move::Paper, Move::Scissors)
            | (Move::Scissors, Move::Rock) => 0,
        };

        result
    }

    pub fn score_with_strategy(&self) -> u64 {
        match &self.player {
            // must lose
            Move::Rock => match &self.opponent {
                Move::Rock => Game {
                    opponent: Move::Rock,
                    player: Move::Scissors,
                },
                Move::Paper => Game {
                    opponent: Move::Paper,
                    player: Move::Rock,
                },
                Move::Scissors => Game {
                    opponent: Move::Scissors,
                    player: Move::Paper,
                },
            },

            // must draw
            Move::Paper => match &self.opponent {
                Move::Rock => Game {
                    opponent: Move::Rock,
                    player: Move::Rock,
                },
                Move::Paper => Game {
                    opponent: Move::Paper,
                    player: Move::Paper,
                },
                Move::Scissors => Game {
                    opponent: Move::Scissors,
                    player: Move::Scissors,
                },
            },

            // must win
            Move::Scissors => match &self.opponent {
                Move::Rock => Game {
                    opponent: Move::Rock,
                    player: Move::Paper,
                },
                Move::Paper => Game {
                    opponent: Move::Paper,
                    player: Move::Scissors,
                },
                Move::Scissors => Game {
                    opponent: Move::Scissors,
                    player: Move::Rock,
                },
            },
        }
        .score()
    }
}

pub fn get_input() -> eyre::Result<Vec<Game>> {
    let input = crate::util::get_input(2022, 2)?;

    let mut results = Vec::new();
    for line in input.lines() {
        let (x, y) = line
            .trim()
            .split_once(' ')
            .ok_or_else(|| eyre::eyre!("{line} was poorly formatted!"))?;

        results.push(Game {
            opponent: x.parse()?,
            player: y.parse()?,
        })
    }

    Ok(results)
}

pub fn part_one(input: &[Game]) -> u64 {
    input.iter().map(|g| g.score()).sum()
}

pub fn part_two(input: &[Game]) -> u64 {
    input.iter().map(|g| g.score_with_strategy()).sum()
}

pub fn main() -> eyre::Result<()> {
    let input = get_input()?;

    tracing::info!("Y2022D02P01: {}", part_one(&input));
    tracing::info!("Y2022D02P02: {}", part_two(&input));

    Ok(())
}

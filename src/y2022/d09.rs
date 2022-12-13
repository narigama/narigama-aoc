use std::{collections::HashSet, str::FromStr};

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl FromStr for Direction {
    type Err = eyre::Error;

    fn from_str(s: &str) -> eyre::Result<Self> {
        match s.trim() {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "R" => Ok(Self::Right),
            "L" => Ok(Self::Left),
            other => Err(eyre::eyre!("unrecognised direction: `{other}`")),
        }
    }
}

#[derive(Debug)]
pub struct Motion {
    direction: Direction,
    steps: i64,
}

impl FromStr for Motion {
    type Err = eyre::Error;

    fn from_str(line: &str) -> eyre::Result<Self> {
        let (direction, steps) = line
            .split_once(' ')
            .ok_or_else(|| eyre::eyre!("bad format for Direction: `{line}`"))?;

        Ok(Self {
            direction: direction.parse()?,
            steps: steps.parse()?,
        })
    }
}

pub fn move_tail(head: (i64, i64), tail: &mut (i64, i64)) {
    let dx = head.0 - tail.0;
    let dy = head.1 - tail.1;

    // move the tail if it's too far away
    if dx.abs() >= 2 || dy.abs() >= 2 {
        // move the tail
        tail.0 += dx.signum();
        tail.1 += dy.signum();
    }
}

pub fn simulate_rope(input: &[Motion], knots: usize) -> u64 {
    let mut rope = vec![(0, 0); knots];

    let mut visited = HashSet::new();

    // for each motion
    for Motion { direction, steps } in input {
        // apply the motion, n times
        (0..*steps).for_each(|_| {
            // move the head of the rope
            match direction {
                Direction::Up => rope[0].1 += 1,
                Direction::Down => rope[0].1 -= 1,
                Direction::Right => rope[0].0 += 1,
                Direction::Left => rope[0].0 -= 1,
            }

            // for each segment in the rope, move the tail if the head moves too far
            (0..rope.len() - 1).for_each(|index| move_tail(rope[index], &mut rope[index + 1]));

            // log the tail position after each step of each motion
            visited.insert(*rope.last().unwrap());
        })
    }

    visited.len() as _
}

pub fn get_input() -> eyre::Result<Vec<Motion>> {
    crate::util::get_input(2022, 9)?
        .lines()
        .map(FromStr::from_str)
        .collect::<eyre::Result<Vec<_>>>()
}

pub fn part_one(input: &[Motion]) -> u64 {
    simulate_rope(input, 2)
}

pub fn part_two(input: &[Motion]) -> u64 {
    simulate_rope(input, 10)
}

pub fn main() -> eyre::Result<()> {
    let input = get_input()?;

    tracing::info!("Y2022D01P01: {}", part_one(&input));
    tracing::info!("Y2022D01P02: {}", part_two(&input));

    Ok(())
}

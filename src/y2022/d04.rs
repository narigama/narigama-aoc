use std::{ops::RangeInclusive, str::FromStr};

#[derive(Debug)]
pub struct Section {
    left: (u64, u64),
    right: (u64, u64),
}

impl Section {
    pub fn range_left(&self) -> RangeInclusive<u64> {
        self.left.0..=self.left.1
    }

    pub fn range_right(&self) -> RangeInclusive<u64> {
        self.right.0..=self.right.1
    }
}

impl FromStr for Section {
    type Err = eyre::Error;

    fn from_str(line: &str) -> eyre::Result<Self> {
        let (left, right) = line
            .split_once(',')
            .ok_or_else(|| eyre::eyre!("{line} is missing a hyphen"))?;

        let (a, b) = left
            .split_once('-')
            .ok_or_else(|| eyre::eyre!("{line} is missing a left comma"))?;

        let (c, d) = right
            .split_once('-')
            .ok_or_else(|| eyre::eyre!("{line} is missing a right comma"))?;

        Ok(Self {
            left: (a.parse()?, b.parse()?),
            right: (c.parse()?, d.parse()?),
        })
    }
}

pub fn get_input() -> eyre::Result<Vec<Section>> {
    Ok(crate::util::get_input(2022, 4)?
        .lines()
        .map(Section::from_str)
        .collect::<eyre::Result<Vec<_>>>()?)
}

pub fn part_one(input: &[Section]) -> u64 {
    input
        .iter()
        .filter(|section| {
            let left = section.range_left();
            let right = section.range_right();

            left.clone().into_iter().all(|n| right.contains(&n))
                || right.clone().into_iter().all(|n| left.contains(&n))
        })
        .count() as _
}

pub fn part_two(input: &[Section]) -> u64 {
    input
        .iter()
        .filter(|section| {
            let left = section.range_left();
            let right = section.range_right();

            left.into_iter().any(|n| right.contains(&n))
        })
        .count() as _
}

pub fn main() -> eyre::Result<()> {
    let input = get_input()?;

    tracing::info!("Y2022D04P01: {}", part_one(&input));
    tracing::info!("Y2022D04P02: {}", part_two(&input));

    Ok(())
}

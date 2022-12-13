use std::collections::HashSet;

pub fn score_letter(c: &char) -> u64 {
    match c.is_ascii_uppercase() {
        true => ((*c as u8) - 38) as _,
        false => ((*c as u8) - 96) as _,
    }
}

pub fn get_input() -> eyre::Result<Vec<String>> {
    Ok(crate::util::get_input(2022, 3)?
        .lines()
        .map(From::from)
        .collect::<Vec<_>>())
}

pub fn part_one(input: &[String]) -> eyre::Result<u64> {
    let mut result = 0;

    for line in input {
        let (a, b) = line.split_at(line.len() / 2);
        let x = a.chars().collect::<HashSet<_>>();
        let y = b.chars().collect::<HashSet<_>>();

        result += &x.intersection(&y).map(score_letter).sum();
    }

    Ok(result)
}

pub fn part_two(input: &[String]) -> eyre::Result<u64> {
    let mut result = 0;

    for chunk in input.chunks(3) {
        let mut it = chunk.iter();

        // grab the first element
        let head = it
            .next()
            .ok_or_else(|| eyre::eyre!("unable to get head of chunk"))?
            .chars()
            .collect::<HashSet<_>>();

        // now grab the rest
        let tail = it
            .map(|line| line.chars().collect::<HashSet<_>>())
            .collect::<Vec<_>>();

        // find the unique char among all three lines
        let unique = tail.iter().fold(head, |i, v| {
            i.intersection(&v).copied().collect::<HashSet<_>>()
        });

        result += unique.iter().map(score_letter).sum::<u64>();
    }

    Ok(result)
}

pub fn main() -> eyre::Result<()> {
    let input = get_input()?;

    tracing::info!("Y2022D03P01: {}", part_one(&input)?);
    tracing::info!("Y2022D03P02: {}", part_two(&input)?);

    Ok(())
}

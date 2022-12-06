use std::collections::HashSet;

pub fn find_unique_window(input: &[char], window_size: usize) -> u64 {
    for index in 0..input.len() - window_size {
        let window = input
            .get(index..index + window_size)
            .unwrap() // SAFETY: bounds are enforced above, won't exceed input.len()
            .iter()
            .map(ToOwned::to_owned)
            .collect::<HashSet<_>>();

        if window.len() == window_size {
            return (window_size + index) as _;
        }
    }

    0
}

pub fn get_input() -> eyre::Result<Vec<char>> {
    Ok(crate::util::get_input(2022, 6)?
        .trim()
        .chars()
        .collect::<Vec<_>>())
}

pub fn part_one(input: &[char]) -> u64 {
    find_unique_window(input, 4)
}

pub fn part_two(input: &[char]) -> u64 {
    find_unique_window(input, 14)
}

pub fn main() -> eyre::Result<()> {
    let input = get_input()?;

    tracing::info!("Y2022D06P01: {}", part_one(&input));
    tracing::info!("Y2022D06P02: {}", part_two(&input));

    Ok(())
}

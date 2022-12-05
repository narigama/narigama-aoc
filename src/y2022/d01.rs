// here's the iterable version
// fn get_input() -> eyre::Result<Vec<u64>> {
//     Ok(crate::util::get_input(2022, 1)?
//         .split("\n\n")
//         .map(|elf| {
//             Ok(elf
//                 .split_whitespace()
//                 .map(|line| Ok(line.parse()?))
//                 .collect::<eyre::Result<Vec<_>>>()?
//                 .iter()
//                 .sum())
//         })
//         .collect::<eyre::Result<Vec<_>>>()?)
// }

pub fn get_input() -> eyre::Result<Vec<u64>> {
    let input = crate::util::get_input(2022, 1)?;

    let mut results = Vec::new();
    for elf in input.split("\n\n") {
        let values = elf
            .split_whitespace()
            .map(|line| Ok(line.parse()?))
            .collect::<eyre::Result<Vec<_>>>()?;

        results.push(values.iter().sum())
    }

    Ok(results)
}

pub fn part_one(input: &[u64]) -> u64 {
    *input.iter().max().unwrap()
}

pub fn part_two(input: &[u64]) -> u64 {
    let mut input = input.to_vec(); // clone required, sorting ahead
    input.sort_unstable();

    input.iter().rev().take(3).sum()
}

pub fn main() -> eyre::Result<()> {
    let input = get_input()?;

    tracing::info!("Y2022D01P01: {}", part_one(&input));
    tracing::info!("Y2022D02P02: {}", part_two(&input));

    Ok(())
}

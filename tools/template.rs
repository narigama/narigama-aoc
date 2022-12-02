pub fn get_input() -> eyre::Result<_> {}

pub fn part_one(input: _) -> _ {}

pub fn part_two(input: _) -> _ {}

pub fn main() -> eyre::Result<()> {
    let input = get_input()?;

    tracing::info!("Y2022D01P01: {}", part_one(&input));
    tracing::info!("Y2022D02P01: {}", part_two(&input));

    Ok(())
}

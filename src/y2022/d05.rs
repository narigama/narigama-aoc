use std::collections::HashMap;

pub type Columns = HashMap<u64, Vec<char>>;

#[derive(Debug, Clone)]
pub struct Command {
    pub quantity: u64,
    pub from: u64,
    pub to: u64,
}

#[derive(Debug, Clone)]
pub struct World {
    pub columns: Columns,
    pub commands: Vec<Command>,
}

impl World {
    /// apply rules in accordance to a cratemover9000 (part 1)
    pub fn cratemover_9000(&mut self) -> eyre::Result<&mut Self> {
        for command in self.commands.iter() {
            for _ in 0..command.quantity {
                // pop items one by one into the destination
                let char = self
                    .columns
                    .get_mut(&command.from)
                    .ok_or_else(|| eyre::eyre!("unrecognised column {}", &command.from))?
                    .pop()
                    .ok_or_else(|| eyre::eyre!("no value to pop from index: {}", &command.from))?;

                self.columns
                    .get_mut(&command.to)
                    .ok_or_else(|| eyre::eyre!("unrecognised column {}", &command.to))?
                    .push(char);
            }
        }

        Ok(self)
    }

    /// apply rules in accordance to a cratemover9001 (part 2)
    pub fn cratemover_9001(&mut self) -> eyre::Result<&mut Self> {
        for command in self.commands.iter() {
            let mut items = Vec::new();

            // pop items into the substack
            for _ in 0..command.quantity {
                let char = self
                    .columns
                    .get_mut(&command.from)
                    .ok_or_else(|| eyre::eyre!("unrecognised column {}", &command.from))?
                    .pop()
                    .ok_or_else(|| eyre::eyre!("no value to pop from index: {}", &command.from))?;
                items.push(char);
            }

            // flip the substack and extend the destination
            self.columns
                .get_mut(&command.to)
                .ok_or_else(|| eyre::eyre!("unrecognised column {}", &command.to))?
                .extend(items.iter().rev());
        }

        Ok(self)
    }

    /// print the current state of the World
    pub fn get_output(&self) -> eyre::Result<String> {
        let mut keys = self.columns.keys().copied().collect::<Vec<_>>();
        keys.sort_unstable();

        let result = keys
            .iter()
            .map(|index| {
                Ok(self
                    .columns
                    .get(index)
                    .ok_or_else(|| eyre::eyre!("unknown index: {index}"))?
                    .last()
                    .ok_or_else(|| eyre::eyre!("column {index} is empty!"))?)
            })
            .collect::<eyre::Result<String>>()?;

        Ok(result)
    }
}

pub fn get_input() -> eyre::Result<World> {
    let input = crate::util::get_input(2022, 5)?;

    // split by the double new line
    let (columns_raw, commands_raw) = input
        .split_once("\n\n")
        .ok_or_else(|| eyre::eyre!("input is poorly formatted"))?;

    // parse columns into a hashmap of vectors
    let mut columns = Columns::new();
    for line in columns_raw.lines().rev().skip(1) {
        for (index, char) in line.chars().skip(1).step_by(4).enumerate() {
            if char != ' ' {
                columns.entry((index + 1) as _).or_default().push(char);
            }
        }
    }

    // parse instructions
    let mut commands = Vec::new();
    for line in commands_raw.lines() {
        let parts = line
            .split(' ')
            .filter_map(|p| match p.parse() {
                Ok(value) => Some(value),
                Err(_) => None,
            })
            .collect::<Vec<_>>();

        commands.push(Command {
            quantity: *parts
                .get(0)
                .ok_or_else(|| eyre::eyre!("unable to build command, missing quantity"))?,

            from: *parts
                .get(1)
                .ok_or_else(|| eyre::eyre!("unable to build command, missing from"))?,

            to: *parts
                .get(2)
                .ok_or_else(|| eyre::eyre!("unable to build command, missing to"))?,
        })
    }

    Ok(World { columns, commands })
}

pub fn part_one(input: &mut World) -> eyre::Result<String> {
    input.cratemover_9000()?.get_output()
}

pub fn part_two(input: &mut World) -> eyre::Result<String> {
    input.cratemover_9001()?.get_output()
}

pub fn main() -> eyre::Result<()> {
    let input = get_input()?;

    tracing::info!("Y2022D05P01: {}", part_one(&mut input.clone())?);
    tracing::info!("Y2022D05P02: {}", part_two(&mut input.clone())?);

    Ok(())
}

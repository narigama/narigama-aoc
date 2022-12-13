use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone)]
pub enum Instruction {
    Noop,
    AddX(i64),
}

#[derive(Debug)]
pub enum Pixel {
    On,
    Off,
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Pixel::On => "#",
                Pixel::Off => " ",
            }
        )
    }
}

impl FromStr for Instruction {
    type Err = eyre::Error;

    fn from_str(s: &str) -> eyre::Result<Self> {
        let mut parts = s.split(' ');

        let cmd = parts
            .next()
            .ok_or_else(|| eyre::eyre!("unable to parse {s} into command"))?
            .trim()
            .to_lowercase();

        match cmd.as_ref() {
            "noop" => Ok(Self::Noop),
            "addx" => Ok(Self::AddX(
                parts
                    .next()
                    .ok_or_else(|| eyre::eyre!("unable to create AddX, no value provided"))?
                    .parse()?,
            )),
            other => Err(eyre::eyre!("unknown operation: `{other}`")),
        }
    }
}

#[derive(Debug)]
pub struct ComputerState {
    pub program_counter: usize,
    pub cycles: usize,
    pub register: i64,
    pub sprite: i64,
    pub pixels: Vec<Pixel>,
}

#[derive(Debug)]
pub struct Computer<C>
where
    C: FnMut(&mut ComputerState),
{
    // program
    pub program: Vec<Instruction>,

    // interrupt handler, called on every cpu cycle
    pub handle_cycle: C,

    // state
    pub state: ComputerState,
}

impl<C> Computer<C>
where
    C: FnMut(&mut ComputerState),
{
    pub fn new(instructions: &[Instruction], handle_cycle: C) -> Self {
        Self {
            program: instructions.to_vec(),
            handle_cycle,
            state: ComputerState {
                program_counter: 0,
                cycles: 0,
                register: 1,
                sprite: 1,
                pixels: Vec::new(),
            },
        }
    }

    pub fn cycle(&mut self) {
        self.state.cycles += 1;
        (self.handle_cycle)(&mut self.state)
    }

    pub fn run(&mut self) -> eyre::Result<()> {
        loop {
            // end of program, stop
            if self.state.program_counter >= self.program.len() {
                break;
            }

            // grab instruction, move program counter forward
            let instruction = self.program[self.state.program_counter].clone();
            self.state.program_counter += 1;

            // run instruction
            match instruction {
                Instruction::Noop => {
                    self.cycle();
                }

                Instruction::AddX(value) => {
                    self.cycle();
                    self.cycle();
                    self.state.register += value;
                    self.state.sprite = self.state.register as _;
                }
            };
        }

        Ok(())
    }
}

pub fn get_input() -> eyre::Result<Vec<Instruction>> {
    crate::util::get_input(2022, 10)?
        .lines()
        .map(FromStr::from_str)
        .collect::<eyre::Result<Vec<_>>>()
}

pub fn part_one(input: &[Instruction]) -> eyre::Result<u64> {
    let mut result = 0;
    let mut computer = Computer::new(input, |state| {
        if state.cycles >= 20 && (state.cycles as u64 - 20).rem_euclid(40) == 0 {
            result += state.cycles as u64 * state.register as u64;
        }
    });

    computer.run()?;

    Ok(result)
}

pub fn part_two(input: &[Instruction]) -> eyre::Result<String> {
    let mut result = Vec::new();
    let mut computer = Computer::new(input, |state| {
        let pixel = match (state.sprite - 1..state.sprite + 2).contains(&(state.pixels.len() as _))
        {
            true => Pixel::On,
            false => Pixel::Off,
        };
        state.pixels.push(pixel);

        if state.cycles.rem_euclid(40) == 0 {
            let row = state
                .pixels
                .iter()
                .map(ToString::to_string)
                .collect::<String>();
            result.push(row);
            state.pixels.clear();
        }
    });
    computer.run()?;

    // lol, there's probably a more graceful way of doing this.
    Ok(result
        .join("\n                                                                         ")
        .into())
}

pub fn main() -> eyre::Result<()> {
    let input = get_input()?;

    tracing::info!("Y2022D10P01: {}", part_one(&input)?);
    tracing::info!("Y2022D10P02: {}", part_two(&input)?);

    Ok(())
}

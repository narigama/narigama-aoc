use std::str::FromStr;

#[derive(Debug)]
pub enum Command {
    // provide the relative change
    ChangeDir(String),

    // list files
    ListDir,

    // here's a file and it's size
    ResultFile(String, u64),

    // there is a directory here
    ResultDir(String),
}

impl FromStr for Command {
    type Err = eyre::Error;

    fn from_str(line: &str) -> eyre::Result<Self> {
        Ok(if line.starts_with("$ cd") {
            // SAFETY: split_at panics when OOB, ensure the line is long enough first
            eyre::ensure!(
                line.len() > 5,
                "cd command didn't include a path to change to."
            );
            let (_, path) = line.split_at(5);
            Command::ChangeDir(path.into())
        } else if line.starts_with("$ ls") {
            Command::ListDir
        } else if line.starts_with("dir") {
            let (_, path) = line
                .split_once(' ')
                .ok_or_else(|| eyre::eyre!("malformed directory response"))?;

            Command::ResultDir(path.into())
        } else {
            // must be a ResultFile
            let (size, filename) = line
                .split_once(" ")
                .ok_or_else(|| eyre::eyre!("malformed file response"))?;

            Command::ResultFile(filename.into(), size.parse()?)
        })
    }
}

pub fn get_input() -> eyre::Result<Vec<u64>> {
    let input = crate::util::get_input(2022, 7)?;
    let commands = input
        .lines()
        .map(FromStr::from_str)
        .collect::<eyre::Result<Vec<_>>>()?;

    let mut result = Vec::new();
    let mut current = Vec::new();

    for command in commands {
        match command {
            Command::ChangeDir(path) => match path.as_ref() {
                ".." => result.push(current.pop().unwrap()),
                _ => current.push(0),
            },
            Command::ResultFile(_, size) => {
                for i in &mut current {
                    *i += size;
                }
            }

            // these other commands are redundant given we assume each file/folder is visited once
            _ => {}
        }
    }

    // the last dir won't "cd", append it here
    result.extend(&current);

    Ok(result)
}

pub fn part_one(input: &[u64]) -> u64 {
    input.iter().filter(|n| **n <= 100000).sum()
}

pub fn part_two(input: &[u64]) -> u64 {
    let max = input.iter().max().unwrap();
    let required = 30000000 - (70000000 - max);
    *input.iter().filter(|n| **n >= required).min().unwrap()
}

pub fn main() -> eyre::Result<()> {
    let input = get_input()?;

    tracing::info!("Y2022D07P01: {}", part_one(&input));
    tracing::info!("Y2022D07P02: {}", part_two(&input));

    Ok(())
}

use std::{collections::HashMap, str::FromStr};

use derive_more::{Deref, DerefMut};

// x,y -> height
#[derive(Debug, Default, Deref, DerefMut)]
pub struct Trees {
    #[deref]
    #[deref_mut]
    map: HashMap<(i64, i64), i64>,

    // store max bounds
    pub max_x: i64,
    pub max_y: i64,
}

impl FromStr for Trees {
    type Err = eyre::Error;

    fn from_str(input: &str) -> eyre::Result<Self> {
        let mut map = HashMap::new();
        let mut max_x = 0;
        let mut max_y = 0;

        for (y, line) in input.lines().enumerate() {
            max_y = y.max(max_y);

            for (x, char) in line.chars().enumerate() {
                max_x = x.max(max_x);

                // parse this char into a numerical value, store it at (x,y)
                let value = char
                    .to_digit(10)
                    .ok_or_else(|| eyre::eyre!("value at ({x},{y}) was not an integer: {char}"))?;
                map.insert((x as _, y as _), value as _);
            }
        }

        Ok(Self {
            map,
            max_x: (max_x + 1) as _,
            max_y: (max_y + 1) as _,
        })
    }
}

impl Trees {
    pub fn find_visible_trees(&self) -> i64 {
        // all walls are visible, subtract four for the overlapping corners
        let mut result = (self.max_x * 2) + (self.max_y * 2) - 4;
        // let mut result = 0;

        // now check the middle for more visible trees
        for x in 1..self.max_x - 1 {
            for y in 1..self.max_y - 1 {
                // take all rows/columns, remove self, ensure remaining coords are shorter than self
                let current = self.get(&(x, y)).unwrap();

                // scan north/south/west/east
                let is_visible = (0..y).all(|j| self.get(&(x, j)).unwrap() < current)
                    || (y + 1..self.max_y).all(|j| self.get(&(x, j)).unwrap() < current)
                    || (0..x).all(|i| self.get(&(i, y)).unwrap() < current)
                    || (x + 1..self.max_x).all(|i| self.get(&(i, y)).unwrap() < current);

                if is_visible {
                    result += 1;
                }
            }
        }

        result
    }

    pub fn find_most_scenic_tree(&self) -> i64 {
        let mut result = 0;

        for x in 0..self.max_x {
            for y in 0..self.max_y {
                let current = self.get(&(x, y)).unwrap();

                // for each tree, scan and measure distance until you hit a tree as tall as the current
                // let north = (0..y).rev().take_while(|j| self.get(&(x, *j)).unwrap() < current).count();
                let mut north = 0;
                for j in (0..y).rev() {
                    north += 1;
                    if self.get(&(x, j)).unwrap() >= current {
                        break
                    }
                }
                // let south = (y+1..self.max_y).take_while(|j| self.get(&(x, *j)).unwrap() < current).count();
                let mut south = 0;
                for j in y+1..self.max_y {
                    south += 1;
                    if self.get(&(x, j)).unwrap() >= current {
                        break
                    }
                }

                // let west = (0..x).rev().take_while(|i| self.get(&(*i, y)).unwrap() < current).count();
                let mut west = 0;
                for i in (0..x).rev() {
                    west += 1;
                    if self.get(&(i, y)).unwrap() >= current {
                        break
                    }
                }

                // let east = (x+1..self.max_x).take_while(|i| self.get(&(*i, y)).unwrap() < current).count();
                let mut east = 0;
                for i in x+1..self.max_x {
                    east += 1;
                    if self.get(&(i, y)).unwrap() >= current {
                        break
                    }
                }

                result = result.max(north * south * west * east)
            }
        }

        result as _
    }
}

pub fn get_input() -> eyre::Result<Trees> {
    let input = crate::util::get_input(2022, 8)?;

    Trees::from_str(&input)
}

pub fn part_one(trees: &Trees) -> i64 {
    trees.find_visible_trees()
}

pub fn part_two(trees: &Trees) -> i64 {
    trees.find_most_scenic_tree()
}

pub fn main() -> eyre::Result<()> {
    let input = get_input()?;

    tracing::info!("Y2022D01P01: {}", part_one(&input));
    tracing::info!("Y2022D01P02: {}", part_two(&input));

    Ok(())
}

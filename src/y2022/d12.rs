use pathfinding::matrix::Matrix;

pub type Pos = (usize, usize);

#[derive(Debug, Clone)]
pub enum Tile {
    Elevation(i64),
    Start,
    End,
}

impl TryFrom<char> for Tile {
    type Error = eyre::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Self::Start),
            'E' => Ok(Self::End),
            'a'..='z' => Ok(Self::Elevation(((value as u8) - 96) as _)), // convert a->z to 1->26
            _ => Err(eyre::eyre!("unknown tile: `{value}`")),
        }
    }
}

impl Tile {
    pub fn height(&self) -> i64 {
        match self {
            Tile::Elevation(v) => *v,
            Tile::Start => 1,
            Tile::End => 26,
        }
    }
}

/// returns two pairs, the Start and the End
pub fn find_start_and_end(input: &Matrix<Tile>) -> (Pos, Pos) {
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (k, v) in input.items() {
        match v {
            Tile::Start => start = k,
            Tile::End => end = k,
            _ => (),
        }
    }

    (start, end)
}

pub fn get_successors(input: &Matrix<Tile>, pos: Pos) -> Vec<(Pos, u64)> {
    input
        .neighbours(pos, false)
        .filter_map(|neighbour| {
            // SAFETY: these are bounds checked
            let a = input.get(pos).unwrap();
            let b = input.get(neighbour).unwrap();

            // change in height must be no greater than 1, but can descend as much as you like
            (b.height() - a.height() <= 1).then_some((neighbour, 1))
        })
        .collect::<Vec<_>>()
}

pub fn manhattan_distance(a: &Pos, b: &Pos) -> u64 {
    (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)) as _
}

pub fn shortest_route(input: &Matrix<Tile>, start: &Pos, end: &Pos) -> Option<u64> {
    let result = pathfinding::directed::astar::astar(
        start,
        |pos| get_successors(input, *pos),
        |pos| manhattan_distance(pos, end),
        |pos| pos == end,
    );

    // return the cost if one was found
    result.map(|(_, cost)| cost)
}

pub fn get_input() -> eyre::Result<Matrix<Tile>> {
    let input = crate::util::get_input(2022, 12)?;

    let rows = input
        .lines()
        .map(|line| {
            Ok(line
                .chars()
                .map(|c| Tile::try_from(c))
                .collect::<eyre::Result<Vec<_>>>()?)
        })
        .collect::<eyre::Result<Vec<_>>>()?;

    Ok(Matrix::from_rows(rows)?)
}

pub fn part_one(input: &Matrix<Tile>) -> eyre::Result<u64> {
    let (start, end) = find_start_and_end(input);
    let cost =
        shortest_route(input, &start, &end).ok_or_else(|| eyre::eyre!("Failed to find a route"))?;

    Ok(cost)
}

pub fn part_two(input: &Matrix<Tile>) -> eyre::Result<u64> {
    let (_, end) = find_start_and_end(input);

    let cost = input
        .items()
        .filter_map(|(start, tile)| match tile {
            Tile::Start | Tile::Elevation(1) => shortest_route(input, &start, &end),
            _ => None,
        })
        .min()
        .ok_or_else(|| eyre::eyre!("Failed to find a route"))?;

    Ok(cost)
}

pub fn main() -> eyre::Result<()> {
    let input = get_input()?;

    tracing::info!("Y2022D01P01: {}", part_one(&input)?);
    tracing::info!("Y2022D01P02: {}", part_two(&input)?);

    Ok(())
}

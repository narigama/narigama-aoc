use itertools::Itertools;

use serde_json::{json, Value};

pub fn get_input() -> eyre::Result<Vec<Value>> {
    let input = crate::util::get_input(2022, 13)?;

    Ok(input
        .trim()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Ok(serde_json::from_str(line)?))
        .collect::<eyre::Result<Vec<_>>>()?)
}

pub fn cmp(left: &Value, right: &Value) -> i64 {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => {
            // SAFETY: the above clause verifies the value is a numeric type
            let x = a.as_i64().unwrap();
            let y = b.as_i64().unwrap();

            match x.cmp(&y) {
                std::cmp::Ordering::Less => -1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            }
        }
        (Value::Array(a), Value::Array(b)) => {
            for pair in a.iter().zip_longest(b.iter()) {
                let result = match pair {
                    itertools::EitherOrBoth::Left(_) => 1,
                    itertools::EitherOrBoth::Right(_) => -1,
                    itertools::EitherOrBoth::Both(x, y) => cmp(x, y),
                };

                if result != 0 {
                    return result;
                }
            }

            0
        }
        (Value::Number(_), Value::Array(_)) => {
            let x = Value::Array(vec![left.clone()]);
            cmp(&x, right)
        }
        (Value::Array(_), Value::Number(_)) => {
            let y = Value::Array(vec![right.clone()]);
            cmp(left, &y)
        }
        (_, _) => return 0, // you shouldn't get here...
    }
}

pub fn part_one(input: &[Value]) -> i64 {
    (0..input.len())
        .tuples()
        .filter_map(|(i, j)| (cmp(&input[i], &input[j]) == -1).then_some((i as i64 + 1) / 2 + 1))
        .sum()
}

pub fn part_two(input: &[Value]) -> i64 {
    let div2 = json!([[2]]);
    let div6 = json!([[6]]);

    let input_sorted = input
        .iter()
        .chain([&div2, &div6])
        .map(ToOwned::to_owned)
        .sorted_unstable_by(|a, b| cmp(a, b).cmp(&0))
        .collect::<Vec<_>>();

    // SAFETY: these elements are confirmed to exist, they were inserted earlier
    let x = input_sorted.iter().position(|i| i == &div2).unwrap();
    let y = input_sorted.iter().position(|i| i == &div6).unwrap();

    // non-zero indexing...
    ((x + 1) * (y + 1)) as _
}

pub fn main() -> eyre::Result<()> {
    let input = get_input()?;

    tracing::info!("Y2022D13P01: {}", part_one(&input));
    tracing::info!("Y2022D13P02: {}", part_two(&input));

    Ok(())
}

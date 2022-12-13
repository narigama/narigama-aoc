use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub enum MonkeyOperation {
    // these are the only operations that appear to be used
    Add(i64),
    Multiply(i64),
    Square,
}

impl TryFrom<(&str, &str)> for MonkeyOperation {
    type Error = eyre::Error;

    fn try_from(value: (&str, &str)) -> eyre::Result<Self> {
        match value {
            ("*", "old") => Ok(Self::Square),
            ("*", v) => Ok(Self::Multiply(v.parse()?)),
            ("+", v) => Ok(Self::Add(v.parse()?)),
            (op, _) => Err(eyre::eyre!("unknown operator: {op}")),
        }
    }
}

#[derive(Debug, Default)]
pub struct MonkeyBuilder {
    items: Option<VecDeque<i64>>,
    operation: Option<MonkeyOperation>,
    test_condition: Option<i64>,
    test_true: Option<usize>,
    test_false: Option<usize>,
}

impl MonkeyBuilder {
    pub fn set_items(&mut self, items: VecDeque<i64>) -> &mut Self {
        self.items = Some(items);
        self
    }

    pub fn set_operation(&mut self, operation: MonkeyOperation) -> &mut Self {
        self.operation = Some(operation);
        self
    }

    pub fn set_test_condition(&mut self, test_condition: i64) -> &mut Self {
        self.test_condition = Some(test_condition);
        self
    }

    pub fn set_test_true(&mut self, test_true: usize) -> &mut Self {
        self.test_true = Some(test_true);
        self
    }

    pub fn set_test_false(&mut self, test_false: usize) -> &mut Self {
        self.test_false = Some(test_false);
        self
    }

    pub fn build(self) -> eyre::Result<Monkey> {
        Ok(Monkey {
            inspections: 0,
            items: self.items.ok_or_else(|| eyre::eyre!("items was not set"))?,
            operation: self
                .operation
                .ok_or_else(|| eyre::eyre!("operation was not set"))?,
            test_condition: self
                .test_condition
                .ok_or_else(|| eyre::eyre!("test_condition was not set"))?,
            test_true: self
                .test_true
                .ok_or_else(|| eyre::eyre!("test_true was not set"))?,
            test_false: self
                .test_false
                .ok_or_else(|| eyre::eyre!("test_false was not set"))?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Monkey {
    pub inspections: u64,
    pub items: VecDeque<i64>,

    // what operation to perform
    pub operation: MonkeyOperation,

    // if divisible by condition...
    pub test_condition: i64,

    // ...move the current item to these indexes
    pub test_true: usize,
    pub test_false: usize,
}

pub fn get_input() -> eyre::Result<Vec<Monkey>> {
    Ok(crate::util::get_input(2022, 11)?
        .split("\n\n")
        .map(|monkey| {
            let mut builder = MonkeyBuilder::default();

            for line in monkey.lines().skip(1).map(|l| l.trim().to_lowercase()) {
                match line.split(' ').collect::<Vec<_>>()[..] {
                    ["starting", "items:", ..] => {
                        builder.set_items(
                            line.split_once(": ")
                                .ok_or_else(|| eyre::eyre!("items line is poorly formatted"))?
                                .1
                                .split(',')
                                .map(|i| Ok(i.trim().parse()?))
                                .collect::<eyre::Result<VecDeque<_>>>()?,
                        );
                    }
                    ["operation:", "new", "=", "old", operator, value] => {
                        builder.set_operation(MonkeyOperation::try_from((operator, value))?);
                    }
                    ["test:", "divisible", "by", value] => {
                        builder.set_test_condition(value.parse()?);
                    }
                    ["if", "true:", "throw", "to", "monkey", index] => {
                        builder.set_test_true(index.parse()?);
                    }
                    ["if", "false:", "throw", "to", "monkey", index] => {
                        builder.set_test_false(index.parse()?);
                    }
                    _ => eyre::bail!("unknown line: {line}"),
                };
            }

            Ok(builder.build()?)
        })
        .collect::<eyre::Result<Vec<_>>>()?)
}

pub fn monkey_business(input: &[Monkey], rounds: u64, worry_factor: i64) -> u64 {
    let mut monkeys = input.to_vec();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            for _ in 0..monkeys[i].items.len() {
                // SAFETY: bounds check above.
                let mut item = monkeys[i].items.pop_front().unwrap();

                // increase inspections
                monkeys[i].inspections += 1;

                // check each item, apply the operation to their worry factor
                item = match &monkeys[i].operation {
                    MonkeyOperation::Add(v) => item + v,
                    MonkeyOperation::Multiply(v) => item * v,
                    MonkeyOperation::Square => item * item,
                };

                // before testing, adjust the item's worry value
                item = match &worry_factor == &0 {
                    true => item % 9699690, // wat
                    false => item / &worry_factor,
                };

                // where should the item be thrown?
                match item % monkeys[i].test_condition == 0 {
                    true => {
                        let index = monkeys[i].test_true;
                        monkeys[index].items.push_back(item);
                    }
                    false => {
                        let index = monkeys[i].test_false;
                        monkeys[index].items.push_back(item);
                    }
                };
            }
        }
    }

    // get the top two active
    monkeys.sort_unstable_by_key(|m| m.inspections);

    monkeys
        .iter()
        .map(|m| m.inspections)
        .rev()
        .take(2)
        .product()
}

pub fn part_one(input: &[Monkey]) -> u64 {
    monkey_business(input, 20, 3.into())
}

pub fn part_two(input: &[Monkey]) -> u64 {
    monkey_business(input, 10000, 0.into())
}

pub fn main() -> eyre::Result<()> {
    let input = get_input()?;

    tracing::info!("Y2022D11P01: {}", part_one(&input));
    tracing::info!("Y2022D11P02: {}", part_two(&input));

    Ok(())
}

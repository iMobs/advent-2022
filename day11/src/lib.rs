use std::str::FromStr;

pub fn challenge_1(input: &str) -> u64 {
    let monkeys = parse_monkeys(input);

    monkey_inspection(monkeys, 20, |item| item / 3)
}

pub fn challenge_2(input: &str) -> u64 {
    let monkeys = parse_monkeys(input);

    let modulo: u64 = monkeys.iter().map(|monkey| monkey.divisor).product();

    monkey_inspection(monkeys, 10_000, |item| item % modulo)
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|input| input.parse())
        .collect::<Result<Vec<Monkey>, _>>()
        .unwrap()
}

fn monkey_inspection<F>(mut monkeys: Vec<Monkey>, rounds: usize, bored_op: F) -> u64
where
    F: Fn(u64) -> u64,
{
    for _round in 0..rounds {
        for m in 0..monkeys.len() {
            while let Some(item) = monkeys[m].items.pop() {
                monkeys[m].inspected += 1;
                let item = monkeys[m].op.calc(item);

                let item = bored_op(item);

                let index = if item % monkeys[m].divisor == 0 {
                    monkeys[m].true_index
                } else {
                    monkeys[m].false_index
                };

                monkeys[index].items.push(item);
            }
        }
    }

    let mut inspected: Vec<u64> = monkeys.iter().map(|monkey| monkey.inspected).collect();
    inspected.sort();
    inspected.reverse();

    inspected.iter().take(2).product()
}

#[derive(Debug, PartialEq)]
enum Op {
    Add(u64),
    Mul(u64),
    Square,
}

impl Op {
    fn calc(&self, old: u64) -> u64 {
        match self {
            Self::Add(v) => old + v,
            Self::Mul(v) => old * v,
            Self::Square => old * old,
        }
    }
}

impl FromStr for Op {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op, value) = s[23..].split_once(' ').unwrap();

        let op = match op {
            "+" => Self::Add(value.parse().unwrap()),
            "*" => match value {
                "old" => Self::Square,
                value => Self::Mul(value.parse().unwrap()),
            },
            _ => unreachable!(),
        };

        Ok(op)
    }
}

#[derive(Debug, PartialEq)]
struct Monkey {
    items: Vec<u64>,
    op: Op,
    divisor: u64,
    true_index: usize,
    false_index: usize,
    inspected: u64,
}

impl FromStr for Monkey {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.lines().skip(1);
        let items = s.next().unwrap()[18..]
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect();
        let op: Op = s.next().unwrap().parse().unwrap();
        let divisor = s.next().unwrap()[21..].parse().unwrap();
        let true_index = s.next().unwrap()[29..].parse().unwrap();
        let false_index = s.next().unwrap()[30..].parse().unwrap();

        Ok(Monkey {
            items,
            op,
            divisor,
            true_index,
            false_index,
            inspected: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn it_parses_monkeys() {
        let result = TEST_INPUT.split("\n\n").collect::<Vec<&str>>()[0].parse();
        assert_eq!(
            result,
            Ok(Monkey {
                items: vec![79, 98],
                op: Op::Mul(19),
                divisor: 23,
                true_index: 2,
                false_index: 3,
                inspected: 0,
            })
        );
    }

    #[test]
    fn it_solves_challege_1() {
        assert_eq!(challenge_1(TEST_INPUT), 10605);
    }

    #[test]
    fn it_solves_challege_2() {
        assert_eq!(challenge_2(TEST_INPUT), 2713310158);
    }
}

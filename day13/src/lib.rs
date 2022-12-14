use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace1, newline},
    combinator::{all_consuming, map, map_res},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};

pub fn challenge_1(input: &str) -> usize {
    let (r, list) = pair_parser(input).unwrap();
    assert!(r.is_empty());
    list.into_iter()
        .enumerate()
        .filter(|(_, (left, right))| left < right)
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn challenge_2(input: &str) -> usize {
    const DIV1: Nested = Nested::Value(2);
    const DIV2: Nested = Nested::Value(6);

    let (r, nested) = list_parser(input).unwrap();
    assert!(r.is_empty());

    let mut div1_index = 1;
    let mut div2_index = 2;
    for nested in nested {
        if nested < DIV1 {
            div1_index += 1;
        }

        if nested < DIV2 {
            div2_index += 1;
        }
    }

    div1_index * div2_index
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Nested {
    List(Vec<Nested>),
    Value(u8),
}

fn nested_parser(input: &str) -> IResult<&str, Nested> {
    map(
        delimited(
            tag("["),
            separated_list0(
                tag(","),
                alt((
                    map(map_res(digit1, |s: &str| s.parse()), Nested::Value),
                    nested_parser,
                )),
            ),
            tag("]"),
        ),
        Nested::List,
    )(input)
}

fn pair_parser(input: &str) -> IResult<&str, Vec<(Nested, Nested)>> {
    all_consuming(separated_list1(
        multispace1,
        separated_pair(nested_parser, newline, nested_parser),
    ))(input)
}

fn list_parser(input: &str) -> IResult<&str, Vec<Nested>> {
    all_consuming(separated_list1(multispace1, nested_parser))(input)
}

#[derive(Debug, PartialEq)]
struct ParseNestedError;

impl PartialOrd for Nested {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Nested {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::List(left), Self::List(right)) => left.iter().cmp(right.iter()),
            (Self::List(left), Self::Value(right)) => left.iter().cmp(&[Self::Value(*right)]),
            (Self::Value(left), Self::List(right)) => [Self::Value(*left)].iter().cmp(right.iter()),
            (Self::Value(left), Self::Value(right)) => left.cmp(right),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_a_list() {
        let list = "[[10],[2,3,4]]";
        let (_, result) = nested_parser(list).unwrap();
        assert_eq!(
            result,
            Nested::List(vec![
                Nested::List(vec![Nested::Value(10)]),
                Nested::List(vec![Nested::Value(2), Nested::Value(3), Nested::Value(4)]),
            ]),
        );
    }

    #[test]
    fn it_checks_order() {
        let left = Nested::List(vec![
            Nested::List(vec![Nested::Value(1)]),
            Nested::List(vec![Nested::Value(2), Nested::Value(3), Nested::Value(4)]),
        ]);
        let right = Nested::List(vec![Nested::List(vec![Nested::Value(1)]), Nested::Value(4)]);

        assert!(left == left);
        assert!(left < right);
    }

    const TEST_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn it_solves_challenge_1() {
        assert_eq!(challenge_1(TEST_INPUT), 13);
    }

    #[test]
    fn it_solves_challenge_2() {
        assert_eq!(challenge_2(TEST_INPUT), 140);
    }
}

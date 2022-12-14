use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, newline},
    combinator::{all_consuming, map_res},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

pub fn challenge_1(input: &str) -> usize {
    let (_, (floor, mut sandbox)) = parse_input(input).unwrap();

    let mut sand_stack = vec![(500, 0)];
    let rocks = sandbox.len();

    loop {
        let (mut sand_x, mut sand_y) = sand_stack.last().unwrap();

        loop {
            if sand_y > floor {
                return sandbox.len() - rocks;
            }

            if sandbox.contains(&(sand_x, sand_y + 1)) {
                if !sandbox.contains(&(sand_x - 1, sand_y + 1)) {
                    sand_x -= 1;
                } else if !sandbox.contains(&(sand_x + 1, sand_y + 1)) {
                    sand_x += 1;
                } else {
                    sand_stack.pop();
                    sandbox.insert((sand_x, sand_y));
                    break;
                }
            }
            sand_y += 1;
            sand_stack.push((sand_x, sand_y));
        }
    }
}

pub fn challenge_2(input: &str) -> usize {
    let (_, (floor, mut sandbox)) = parse_input(input).unwrap();

    let floor = floor + 2;

    let mut sand_stack = vec![(500, 0)];
    let rocks = sandbox.len();

    loop {
        let (mut sand_x, mut sand_y) = match sand_stack.last() {
            Some(v) => v,
            None => return sandbox.len() - rocks,
        };

        loop {
            if sand_y + 1 == floor {
                sand_stack.pop();
                sandbox.insert((sand_x, sand_y));
                break;
            }

            if sandbox.contains(&(sand_x, sand_y + 1)) {
                if !sandbox.contains(&(sand_x - 1, sand_y + 1)) {
                    sand_x -= 1;
                } else if !sandbox.contains(&(sand_x + 1, sand_y + 1)) {
                    sand_x += 1;
                } else {
                    sand_stack.pop();
                    sandbox.insert((sand_x, sand_y));
                    break;
                }
            }
            sand_y += 1;
            sand_stack.push((sand_x, sand_y));
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, (u32, HashSet<(u32, u32)>)> {
    let (r, input) = all_consuming(separated_list1(
        newline,
        separated_list1(
            tag(" -> "),
            separated_pair(parse_number, char(','), parse_number),
        ),
    ))(input)?;

    let mut sandbox = HashSet::new();
    let mut floor = 0;
    for row in input {
        for window in row.as_slice().windows(2) {
            let start = window[0];
            let end = window[1];

            floor = floor.max(start.1).max(end.1);

            if start.0 == end.0 {
                // Vertical line
                let x = start.0;
                let min = start.1.min(end.1);
                let max = start.1.max(end.1);
                for y in min..=max {
                    sandbox.insert((x, y));
                }
            } else if start.1 == end.1 {
                // Horizontal line
                let y = start.1;
                let min = start.0.min(end.0);
                let max = start.0.max(end.0);
                for x in min..=max {
                    sandbox.insert((x, y));
                }
            } else {
                panic!()
            }
        }
    }

    Ok((r, (floor, sandbox)))
}

fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse())(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn it_solves_challenge_1() {
        assert_eq!(challenge_1(TEST_INPUT), 24);
    }

    #[test]
    fn it_solves_challenge_2() {
        assert_eq!(challenge_2(TEST_INPUT), 93);
    }
}

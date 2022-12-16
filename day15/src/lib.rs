use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{i32, newline},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    Finish, IResult,
};

pub fn challenge_1(input: &str) -> i32 {
    get_empty_for_y(input, 2_000_000)
}

const FREQUENCY: usize = 4_000_000;

pub fn challenge_2(input: &str) -> usize {
    get_frequency_in_range(input, 0, FREQUENCY as i32)
}

fn get_empty_for_y(input: &str, y: i32) -> i32 {
    let (_, packets) = parse_input(input).finish().unwrap();

    let mut ranges: Vec<Range> = packets
        .into_iter()
        .map(Packet::to_radius)
        .filter_map(|radius| {
            let y_diff = radius.center.y.abs_diff(y);
            if y_diff <= radius.range {
                let distance = radius.range - y_diff;
                let min_x = radius.center.x - distance as i32;
                let max_x = radius.center.x + distance as i32;
                Some(Range(min_x, max_x))
            } else {
                None
            }
        })
        .collect();

    ranges.sort_unstable();

    let mut collapsed_ranges: Vec<Range> = Vec::new();
    for range in ranges {
        if let Some(last) = collapsed_ranges.last() {
            if let Some(range) = last.collapse(&range) {
                collapsed_ranges.pop();
                collapsed_ranges.push(range);
                continue;
            }
        }
        collapsed_ranges.push(range);
    }

    collapsed_ranges
        .into_iter()
        .map(|Range(low, high)| high - low)
        .sum()
}

fn get_frequency_in_range(input: &str, start: i32, end: i32) -> usize {
    let (_, packets) = parse_input(input).finish().unwrap();

    let list: Vec<Radius> = packets.into_iter().map(Packet::to_radius).collect();

    let mut coords: HashSet<Coord> = HashSet::new();
    for (i, a) in list.iter().enumerate() {
        for b in list.iter().skip(i + 1) {
            coords.extend(
                a.intersection(b)
                    .into_iter()
                    .flat_map(|coord| {
                        let Coord { x, y } = coord;
                        [
                            Coord { x: x - 1, y },
                            Coord { x: x + 1, y },
                            Coord { x, y: y - 1 },
                            Coord { x, y: y + 1 },
                        ]
                    })
                    .filter(|coord| {
                        coord.x >= start && coord.y >= start && coord.x <= end && coord.y <= end
                    }),
            );
        }
    }

    let coord = coords
        .into_iter()
        .find(|coord| {
            list.iter()
                .all(|radius| radius.range < radius.center.manhattan(coord))
        })
        .unwrap();

    coord.frequency()
}

fn parse_input(input: &str) -> IResult<&str, Vec<Packet>> {
    all_consuming(separated_list1(newline, parse_packet))(input)
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    map(
        separated_pair(
            preceded(tag("Sensor at "), parse_position),
            tag(": "),
            preceded(tag("closest beacon is at "), parse_position),
        ),
        |(sensor, beacon)| Packet { sensor, beacon },
    )(input)
}

fn parse_position(input: &str) -> IResult<&str, Coord> {
    map(
        separated_pair(
            preceded(tag("x="), i32),
            tag(", "),
            preceded(tag("y="), i32),
        ),
        |(x, y)| Coord { x, y },
    )(input)
}

#[derive(Debug, Clone, Copy)]
struct Packet {
    sensor: Coord,
    beacon: Coord,
}

impl Packet {
    fn manhattan(&self) -> u32 {
        self.sensor.manhattan(&self.beacon)
    }

    fn to_radius(self) -> Radius {
        Radius {
            center: self.sensor,
            range: self.manhattan(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Range(i32, i32);

impl Range {
    fn collapse(&self, other: &Self) -> Option<Self> {
        if self.0 <= other.1 && other.0 <= self.1 {
            Some(Range(self.0.min(other.0), self.1.max(other.1)))
        } else {
            None
        }
    }
}

struct Radius {
    center: Coord,
    range: u32,
}

impl Radius {
    fn intersection(&self, other: &Self) -> Vec<Coord> {
        let distance = self.center.manhattan(&other.center);
        if distance > self.range + other.range {
            return Vec::new();
        }

        let [a1, b1, c1, d1] = self.sides();
        let [a2, b2, c2, d2] = other.sides();

        let crosspoints = [
            a1.crosspoint(b2),
            a1.crosspoint(c2),
            b1.crosspoint(a2),
            b1.crosspoint(d2),
            c1.crosspoint(a2),
            c1.crosspoint(d2),
            d1.crosspoint(b2),
            d1.crosspoint(c2),
        ];

        crosspoints
            .into_iter()
            .filter_map(|(x, y)| {
                let x = x.round() as i32;
                let y = y.round() as i32;

                let coord = Coord { x, y };
                if coord.manhattan(&self.center) <= self.range
                    && coord.manhattan(&other.center) <= other.range
                {
                    Some(coord)
                } else {
                    None
                }
            })
            .collect()
    }

    fn sides(&self) -> [Coord; 4] {
        let Self { center, range } = self;
        let left = Coord {
            x: center.x - *range as i32,
            y: center.y,
        };
        let right = Coord {
            x: center.x + *range as i32,
            y: center.y,
        };

        [
            Coord {
                x: 1,
                y: -left.x + left.y,
            },
            Coord {
                x: -1,
                y: right.x + right.y,
            },
            Coord {
                x: -1,
                y: left.x + left.y,
            },
            Coord {
                x: 1,
                y: -right.x + right.y,
            },
        ]
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn crosspoint(self, other: Self) -> (f32, f32) {
        let x = (other.y - self.y) as f32 / (self.x - other.x) as f32;
        let y = self.x as f32 * x + self.y as f32;
        (x, y)
    }

    fn manhattan(&self, other: &Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    fn frequency(&self) -> usize {
        self.x as usize * FREQUENCY + self.y as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    const TEST_FREQUENCY: usize = 56_000_011;

    #[test]
    fn it_solves_challenge_1() {
        assert_eq!(get_empty_for_y(TEST_INPUT, 10), 26);
    }

    #[test]
    fn it_solves_challenge_2() {
        assert_eq!(get_frequency_in_range(TEST_INPUT, 0, 20), TEST_FREQUENCY);
    }
}

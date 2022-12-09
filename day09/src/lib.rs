use std::collections::HashSet;

pub fn challenge_1(input: &str) -> usize {
    rope_chase(input, 2)
}

pub fn challenge_2(input: &str) -> usize {
    rope_chase(input, 10)
}

fn rope_chase(input: &str, rope_length: usize) -> usize {
    let mut rope = vec![Position::new(); rope_length];

    let mut unique_tails = HashSet::new();
    unique_tails.insert(*rope.last().unwrap());

    for movement in input.lines().flat_map(Position::iter_from_string) {
        let head = rope.first_mut().unwrap();
        head.x += movement.x;
        head.y += movement.y;

        for i in 0..rope.len() - 1 {
            let head = rope[i];
            let tail = &mut rope[i + 1];
            tail.chase(head);
        }

        unique_tails.insert(*rope.last().unwrap());
    }

    unique_tails.len()
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new() -> Self {
        Self::default()
    }

    fn iter_from_string(input: &str) -> impl Iterator<Item = Position> + '_ {
        let (direction, amount) = input.split_once(' ').expect("unknown movement format");
        let amount = amount.parse().expect("unknown movement amount");
        (0..amount).map(move |_| match direction {
            "R" => Self { x: 1, y: 0 },
            "L" => Self { x: -1, y: 0 },
            "U" => Self { x: 0, y: 1 },
            "D" => Self { x: 0, y: -1 },
            _ => panic!("unknown movement direction"),
        })
    }

    fn chase(&mut self, other: Self) {
        let h_diff = other.x - self.x;
        let v_diff = other.y - self.y;
        let abs_h_diff = h_diff.abs();
        let abs_v_diff = v_diff.abs();

        if abs_h_diff > 1 || abs_v_diff > 1 {
            if abs_h_diff != 0 {
                self.x += h_diff / abs_h_diff;
            }
            if abs_v_diff != 0 {
                self.y += v_diff / abs_v_diff;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn it_solves_challenge_1() {
        assert_eq!(challenge_1(TEST_INPUT), 13);
    }

    #[test]
    fn it_solves_challenge_2() {
        assert_eq!(challenge_2(TEST_INPUT), 1);

        let longer_input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

        assert_eq!(challenge_2(longer_input), 36);
    }
}

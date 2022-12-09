use std::collections::HashSet;

pub fn challenge_1(input: &str) -> usize {
    let mut head = Position::new();
    let mut tail = Position::new();

    let mut unique_tails = HashSet::new();
    unique_tails.insert(tail);

    for movement in input.lines().flat_map(Position::from_string) {
        head += movement;

        tail.chase(head);

        unique_tails.insert(tail);
    }

    unique_tails.len()
}

pub fn challenge_2(input: &str) -> usize {
    let mut rope = [Position::new(); 10];

    let mut unique_tails = HashSet::new();
    unique_tails.insert(*rope.last().unwrap());

    for movement in input.lines().flat_map(Position::from_string) {
        *rope.first_mut().unwrap() += movement;

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

    fn from_string(input: &str) -> impl Iterator<Item = Position> + '_ {
        let (direction, amount) = input.split_once(' ').expect("unknown position format");
        let amount = amount.parse().unwrap();
        (0..amount).map(move |_| match direction {
            "R" => Self { x: 1, y: 0 },
            "L" => Self { x: -1, y: 0 },
            "U" => Self { x: 0, y: 1 },
            "D" => Self { x: 0, y: -1 },
            _ => panic!("unknown direction"),
        })
    }

    fn chase(&mut self, other: Self) {
        let h_diff = other.x - self.x;
        let v_diff = other.y - self.y;
        let abs_h_diff = h_diff.abs();
        let abs_v_diff = v_diff.abs();

        if abs_h_diff > 1 || abs_v_diff > 1 {
            if self.x != other.x && self.y != other.y {
                self.x += h_diff / abs_h_diff;
                self.y += v_diff / abs_v_diff;
            } else if abs_h_diff > 1 {
                self.x += h_diff / abs_h_diff;
            } else {
                self.y += v_diff / abs_v_diff;
            }
        }
    }
}

impl std::ops::Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
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

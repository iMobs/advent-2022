#[derive(Copy, Clone, Debug, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn play(&self, other: Hand) -> i32 {
        self.compare(other) + self.score()
    }

    fn compare(&self, other: Hand) -> i32 {
        if *self == other {
            return 3;
        }

        match (self, other) {
            (Hand::Rock, Hand::Scissors)
            | (Hand::Paper, Hand::Rock)
            | (Hand::Scissors, Hand::Paper) => 6,
            _ => 0,
        }
    }

    fn score(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl From<char> for Hand {
    fn from(input: char) -> Self {
        match input {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!("bad hand!"),
        }
    }
}

enum Result {
    Lose,
    Draw,
    Win,
}

impl From<char> for Result {
    fn from(input: char) -> Self {
        match input {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => panic!("bad result!"),
        }
    }
}

pub fn challenge_1(input: &[&str]) -> i32 {
    get_hands(input).iter().map(|(a, b)| b.play(*a)).sum()
}

fn get_hands(input: &[&str]) -> Vec<(Hand, Hand)> {
    input
        .iter()
        .map(|line| {
            let line: Vec<char> = line.chars().collect();

            (line[0].into(), line[2].into())
        })
        .collect()
}

pub fn challenge_2(input: &[&str]) -> i32 {
    input
        .iter()
        .map(|line| {
            let line: Vec<char> = line.chars().collect();

            let hand: Hand = line[0].into();
            let result: Result = line[2].into();

            let (hand, score) = match result {
                Result::Lose => (
                    match hand {
                        Hand::Rock => Hand::Scissors,
                        Hand::Paper => Hand::Rock,
                        Hand::Scissors => Hand::Paper,
                    },
                    0,
                ),
                Result::Draw => (hand, 3),
                Result::Win => (
                    match hand {
                        Hand::Rock => Hand::Paper,
                        Hand::Paper => Hand::Scissors,
                        Hand::Scissors => Hand::Rock,
                    },
                    6,
                ),
            };

            hand.score() + score
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: [&str; 3] = ["A Y", "B X", "C Z"];

    #[test]
    fn it_parses_hands() {
        assert_eq!(Hand::from('A'), Hand::Rock);
        assert_eq!(Hand::from('X'), Hand::Rock);
        assert_eq!(Hand::from('B'), Hand::Paper);
        assert_eq!(Hand::from('Y'), Hand::Paper);
        assert_eq!(Hand::from('C'), Hand::Scissors);
        assert_eq!(Hand::from('Z'), Hand::Scissors);
    }

    #[test]
    fn it_gets_hands() {
        assert_eq!(
            get_hands(&TEST_INPUT),
            [
                (Hand::Rock, Hand::Paper),
                (Hand::Paper, Hand::Rock),
                (Hand::Scissors, Hand::Scissors),
            ],
        )
    }

    #[test]
    fn it_sums_score() {
        let result = challenge_1(&TEST_INPUT);
        assert_eq!(result, 15);
    }

    #[test]
    fn it_sums_strategy() {
        let result = challenge_2(&TEST_INPUT);
        assert_eq!(result, 12);
    }
}

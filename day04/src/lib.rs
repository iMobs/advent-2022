use std::ops::RangeInclusive;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn split_pairs<'a>(
    input: &'a str,
) -> impl Iterator<Item = (RangeInclusive<i32>, RangeInclusive<i32>)> + 'a {
    input.lines().map(|line| {
        let mut line = line.split(',').map(|range| {
            let mut values = range.split('-').map(|v| v.parse().unwrap());
            values.next().unwrap()..=values.next().unwrap()
        });

        (line.next().unwrap(), line.next().unwrap())
    })
}

pub fn challenge_1(input: &str) -> usize {
    split_pairs(input)
        .filter(|(a, b)| {
            a.start() <= b.start() && b.end() <= a.end()
                || b.start() <= a.start() && a.end() <= b.end()
        })
        .count()
}

pub fn challenge_2(input: &str) -> usize {
    split_pairs(input)
        .filter(|(a, b)| {
            if a.start() <= b.start() {
                b.start() <= a.end()
            } else {
                a.start() <= b.end()
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn it_splits_pairs() {
        let result: Vec<_> = split_pairs(TEST_INPUT).collect();
        assert_eq!(
            result,
            [
                (2..=4, 6..=8),
                (2..=3, 4..=5),
                (5..=7, 7..=9),
                (2..=8, 3..=7),
                (6..=6, 4..=6),
                (2..=6, 4..=8),
            ]
        );
    }

    #[test]
    fn it_checks_overlaps() {
        let result = challenge_1(TEST_INPUT);
        assert_eq!(result, 2);
    }

    #[test]
    fn it_checks_all_overlaps() {
        let result = challenge_2(TEST_INPUT);
        assert_eq!(result, 4);
    }
}

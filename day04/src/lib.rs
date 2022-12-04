use std::ops::RangeInclusive;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn split_pairs(input: &[&str]) -> Vec<(RangeInclusive<i32>, RangeInclusive<i32>)> {
    input
        .iter()
        .map(|line| {
            let line: Vec<RangeInclusive<i32>> = line
                .split(',')
                .map(|range| {
                    let values: Vec<i32> = range.split('-').map(|v| v.parse().unwrap()).collect();
                    values[0]..=values[1]
                })
                .collect();

            (line[0].clone(), line[1].clone())
        })
        .collect()
}

pub fn work_overlaps(input: &[&str]) -> usize {
    split_pairs(input)
        .into_iter()
        .filter(|(a, b)| {
            a.start() <= b.start() && b.end() <= a.end()
                || b.start() <= a.start() && a.end() <= b.end()
        })
        .count()
}

pub fn any_overlaps(input: &[&str]) -> usize {
    split_pairs(input)
        .into_iter()
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

    const TEST_INPUT: [&str; 6] = [
        "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
    ];

    #[test]
    fn it_splits_pairs() {
        let result = split_pairs(&TEST_INPUT);
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
        let result = work_overlaps(&TEST_INPUT);
        assert_eq!(result, 2);
    }

    #[test]
    fn it_checks_all_overlaps() {
        let result = any_overlaps(&TEST_INPUT);
        assert_eq!(result, 4);
    }
}

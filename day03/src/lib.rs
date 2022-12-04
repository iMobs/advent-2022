use std::collections::HashSet;

fn split_pack(pack: &str) -> (&str, &str) {
    let half = pack.len() / 2;

    (&pack[..half], &pack[half..])
}

fn get_overlap(pack: &str) -> HashSet<char> {
    let (left, right) = split_pack(pack);
    let left: HashSet<char> = left.chars().collect();
    let right: HashSet<char> = right.chars().collect();

    &left & &right
}

fn get_priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => panic!("ruh roh raggy"),
    }
}

pub fn sum_priorities(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|pack| get_overlap(pack).into_iter().map(get_priority).sum::<u32>())
        .sum()
}

pub fn sum_group_priorities(input: &[&str]) -> u32 {
    input
        .chunks(3)
        .map(|packs| {
            packs
                .iter()
                .map(|pack| pack.chars().collect::<HashSet<char>>())
                .reduce(|acc, pack| &acc & &pack)
                .unwrap()
                .into_iter()
                .map(get_priority)
                .sum::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: [&str; 6] = [
        "vJrwpWtwJgWrhcsFMMfFFhFp",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        "PmmdzqPrVvPwwTWBwg",
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
        "ttgJtRGJQctTZtZT",
        "CrZsJsPPZsGzwwsLwLmpwMDw",
    ];

    #[test]
    fn it_splits_pack() {
        let result = split_pack(TEST_INPUT[0]);
        assert_eq!(result, ("vJrwpWtwJgWr", "hcsFMMfFFhFp"));
    }

    #[test]
    fn it_gets_overlap() {
        assert_eq!(get_overlap(TEST_INPUT[0]), HashSet::from(['p']));
        assert_eq!(get_overlap(TEST_INPUT[1]), HashSet::from(['L']));
        assert_eq!(get_overlap(TEST_INPUT[2]), HashSet::from(['P']));
        assert_eq!(get_overlap(TEST_INPUT[3]), HashSet::from(['v']));
        assert_eq!(get_overlap(TEST_INPUT[4]), HashSet::from(['t']));
        assert_eq!(get_overlap(TEST_INPUT[5]), HashSet::from(['s']));
    }

    #[test]
    fn it_gets_priority() {
        assert_eq!(get_priority('a'), 1);
        assert_eq!(get_priority('z'), 26);
        assert_eq!(get_priority('A'), 27);
        assert_eq!(get_priority('Z'), 52);
    }

    #[test]
    fn it_sums_priorities() {
        assert_eq!(sum_priorities(&TEST_INPUT), 157);
    }

    #[test]
    fn it_sums_group_priorities() {
        assert_eq!(sum_group_priorities(&TEST_INPUT), 70);
    }
}

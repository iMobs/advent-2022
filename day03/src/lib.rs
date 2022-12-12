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

pub fn challenge_1(input: &str) -> u32 {
    input
        .lines()
        .map(|pack| get_overlap(pack).into_iter().map(get_priority).sum::<u32>())
        .sum()
}

pub fn challenge_2(input: &str) -> u32 {
    input
        .lines()
        .collect::<Vec<&str>>()
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

    const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn it_gets_priority() {
        assert_eq!(get_priority('a'), 1);
        assert_eq!(get_priority('z'), 26);
        assert_eq!(get_priority('A'), 27);
        assert_eq!(get_priority('Z'), 52);
    }

    #[test]
    fn it_sums_priorities() {
        assert_eq!(challenge_1(TEST_INPUT), 157);
    }

    #[test]
    fn it_sums_group_priorities() {
        assert_eq!(challenge_2(TEST_INPUT), 70);
    }
}

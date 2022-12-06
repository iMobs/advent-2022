use std::collections::{HashSet, VecDeque};

pub fn challenge_1(input: &str) -> usize {
    find_unique_limit(input, 4)
}

pub fn challenge_2(input: &str) -> usize {
    alt_find_unique_limit(input, 14)
}

fn find_unique_limit(input: &str, limit: usize) -> usize {
    let mut queue = VecDeque::new();

    for (i, c) in input.chars().enumerate() {
        queue.push_back(c);

        if queue.len() == limit {
            let set: HashSet<_> = queue.iter().collect();
            if set.len() == limit {
                return i + 1;
            }

            queue.pop_front();
        }
    }

    unreachable!()
}

fn alt_find_unique_limit(input: &str, limit: usize) -> usize {
    input
        .as_bytes()
        .windows(limit)
        .position(|w| {
            let set: HashSet<_> = w.iter().collect();
            set.len() == limit
        })
        .unwrap()
        + limit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_challenge_1() {
        assert_eq!(challenge_1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(challenge_1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(challenge_1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(challenge_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(challenge_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_challenge_2() {
        assert_eq!(challenge_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(challenge_2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(challenge_2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(challenge_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(challenge_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}

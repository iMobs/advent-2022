pub fn challenge_1(input: &str) -> u32 {
    *sum_calories(input).iter().max().unwrap()
}

pub fn challenge_2(input: &str) -> u32 {
    let mut list = sum_calories(input);
    list.sort();

    list.iter().rev().take(3).sum()
}

fn sum_calories(list: &str) -> Vec<u32> {
    let mut result = Vec::new();
    let mut current = 0;

    for line in list.lines() {
        if line.is_empty() {
            result.push(current);
            current = 0;
        } else {
            let item: u32 = line.parse().unwrap();
            current += item;
        }
    }

    if current != 0 {
        result.push(current);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn it_finds_the_most_calories() {
        let result = challenge_1(TEST_INPUT);
        assert_eq!(result, 24_000);
    }

    #[test]
    fn it_sums_the_top_3_calories() {
        let result = challenge_2(TEST_INPUT);
        assert_eq!(result, 45_000);
    }

    #[test]
    fn it_sums_calories() {
        let result = sum_calories(TEST_INPUT);
        assert_eq!(result, [6_000, 4_000, 11_000, 24_000, 10_000]);
    }
}

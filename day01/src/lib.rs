pub fn most_calories(list: &[&str]) -> i32 {
    *sum_calories(list).iter().max().unwrap()
}

pub fn top_3_calories(list: &[&str]) -> i32 {
    let mut list = sum_calories(list);
    list.sort();

    list.iter().rev().take(3).sum()
}

fn sum_calories(list: &[&str]) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut current = 0;

    for item in list {
        if item.is_empty() {
            result.push(current);

            current = 0;
            continue;
        }

        let item: i32 = item.parse().unwrap();
        current += item;
    }

    if current != 0 {
        result.push(current);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: [&str; 14] = [
        "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
        "10000",
    ];

    #[test]
    fn it_finds_the_most_calories() {
        let result = most_calories(&TEST_INPUT);
        assert_eq!(result, 24_000);
    }

    #[test]
    fn it_sums_the_top_3_calories() {
        let result = top_3_calories(&TEST_INPUT);
        assert_eq!(result, 45_000);
    }

    #[test]
    fn it_sums_calories() {
        let result = sum_calories(&TEST_INPUT);
        assert_eq!(result, [6_000, 4_000, 11_000, 24_000, 10_000]);
    }
}

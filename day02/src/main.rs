use day02::{sum_score, sum_strategy};

fn main() {
    let input = include_str!("../input.txt");
    let input: Vec<&str> = input.trim().split('\n').collect();
    let result = sum_score(&input);
    println!("part1: {result}");

    let result = sum_strategy(&input);
    println!("part2: {result}");
}

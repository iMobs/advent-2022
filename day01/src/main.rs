use day01::{most_calories, top_3_calories};

fn main() {
    let input = include_str!("../input.txt");
    let input: Vec<&str> = input.trim().lines().collect();
    let result = most_calories(&input);
    println!("part1: {result}");

    let result = top_3_calories(&input);
    println!("part2: {result}");
}

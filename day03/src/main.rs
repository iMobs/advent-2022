use day03::{sum_group_priorities, sum_priorities};

fn main() {
    let input = include_str!("../input.txt");
    let input: Vec<&str> = input.trim().split('\n').collect();
    let result = sum_priorities(&input);
    println!("part1: {result}");

    let result = sum_group_priorities(&input);
    println!("Part2: {result}");
}

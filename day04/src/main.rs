use day04::{any_overlaps, work_overlaps};

fn main() {
    let input = include_str!("../input.txt");
    let input: Vec<&str> = input.trim().lines().collect();
    let result = work_overlaps(&input);
    println!("part1: {result}");

    let result = any_overlaps(&input);
    println!("part2: {result}");
}

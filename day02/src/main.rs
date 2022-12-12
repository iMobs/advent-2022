use day02::{challenge_1, challenge_2};

fn main() {
    let input = include_str!("../input.txt");
    let result = challenge_1(input);
    println!("part1: {result}");

    let result = challenge_2(input);
    println!("part2: {result}");
}

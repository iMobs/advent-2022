use day05::{invert_crates, stack_chunks};

fn main() {
    let input = include_str!("../input.txt").trim_end();
    let result = invert_crates(input);
    println!("part1: {result}");

    let result = stack_chunks(input);
    println!("part2: {result}");
}

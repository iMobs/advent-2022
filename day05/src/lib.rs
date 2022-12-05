use std::collections::HashMap;

fn parse_stacks(stacks: &str) -> HashMap<usize, Vec<&str>> {
    let mut stacks = stacks.lines().rev();
    let mut stack_map: HashMap<usize, Vec<&str>> = HashMap::new();

    let ids = stacks.next().unwrap();
    for id in ids.split_ascii_whitespace() {
        stack_map.insert(id.parse().unwrap(), Vec::new());
    }

    for mut row in stacks {
        let mut index = 0;
        while !row.is_empty() {
            index += 1;
            let b = &row[..3].trim();
            row = &row[3..];

            if !row.is_empty() {
                row = &row[1..];
            }

            if b.is_empty() {
                continue;
            }
            let list = stack_map.get_mut(&index).unwrap();
            list.push(&b[1..2]);
        }
    }

    stack_map
}

fn parse_moves(moves: &str) -> impl Iterator<Item = (usize, usize, usize)> + '_ {
    moves.lines().map(|line| {
        let mut line = line.split_ascii_whitespace();
        let _move = line.next();
        let quantity: usize = line.next().unwrap().parse().unwrap();
        let _from = line.next();
        let start: usize = line.next().unwrap().parse().unwrap();
        let _to = line.next();
        let end: usize = line.next().unwrap().parse().unwrap();

        (quantity, start, end)
    })
}

pub fn invert_crates(input: &str) -> String {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let stacks = parts[0];
    let moves = parts[1];

    let mut stack_map = parse_stacks(stacks);
    for (quantity, start, end) in parse_moves(moves) {
        for _ in 0..quantity {
            let b = stack_map.get_mut(&start).unwrap().pop().unwrap();
            stack_map.get_mut(&end).unwrap().push(b);
        }
    }

    let mut result = String::new();

    for i in 1..=stack_map.keys().len() {
        let stack = stack_map.get(&i).unwrap();
        let b = stack.last().unwrap();
        result.push_str(b);
    }

    result
}

pub fn stack_chunks(input: &str) -> String {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let stacks = parts[0];
    let moves = parts[1];

    let mut stack_map = parse_stacks(stacks);
    for (quantity, start, end) in parse_moves(moves) {
        let mut tmp = Vec::new();
        for _ in 0..quantity {
            let b = stack_map.get_mut(&start).unwrap().pop().unwrap();
            tmp.push(b);
        }

        while let Some(b) = tmp.pop() {
            stack_map.get_mut(&end).unwrap().push(b);
        }
    }

    let mut result = String::new();

    for i in 1..=stack_map.keys().len() {
        let stack = stack_map.get(&i).unwrap();
        let b = stack.last().unwrap();
        result.push_str(b);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn it_works() {
        let result = invert_crates(TEST_INPUT);
        assert_eq!(result, "CMZ")
    }
}

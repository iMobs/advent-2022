use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

pub fn challenge_1(input: &str) -> usize {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let input: Vec<Vec<u8>> = input
        .lines()
        .enumerate()
        .map(|(x, line)| {
            line.bytes()
                .enumerate()
                .map(|(y, c)| {
                    if c == b'S' {
                        start = (x, y);
                        b'a'
                    } else if c == b'E' {
                        end = (x, y);
                        b'z'
                    } else {
                        c
                    }
                })
                .collect()
        })
        .collect();

    pathfinder(input, vec![start], end)
}

pub fn challenge_2(input: &str) -> usize {
    let mut start = Vec::new();
    let mut end = (0, 0);
    let input: Vec<Vec<u8>> = input
        .lines()
        .enumerate()
        .map(|(x, line)| {
            line.bytes()
                .enumerate()
                .map(|(y, c)| {
                    if c == b'a' || c == b'S' {
                        start.push((x, y));
                        b'a'
                    } else if c == b'E' {
                        end = (x, y);
                        b'z'
                    } else {
                        c
                    }
                })
                .collect()
        })
        .collect();

    pathfinder(input, start, end)
}

fn pathfinder(input: Vec<Vec<u8>>, start: Vec<(usize, usize)>, end: (usize, usize)) -> usize {
    let x_len = input.len();
    let y_len = input[0].len();

    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();

    for (x, y) in start {
        queue.push((Reverse(manhattan((x, y), end)), Reverse(0), x, y));
    }

    while let Some((Reverse(_score), Reverse(steps), x, y)) = queue.pop() {
        if (x, y) == end {
            return steps;
        }

        if visited.contains(&(x, y)) {
            continue;
        }

        visited.insert((x, y));

        let h = input[x][y];

        for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let dx = x as isize + dx;
            let dy = y as isize + dy;

            if dx < 0 || dy < 0 {
                continue;
            }

            let dx = dx as usize;
            let dy = dy as usize;
            if dx >= x_len || dy >= y_len {
                continue;
            }

            if visited.contains(&(dx, dy)) {
                continue;
            }

            let dh = input[dx][dy];

            if dh > h && dh - h > 1 {
                continue;
            }

            let steps = steps + 1;
            let score = manhattan((dx, dy), end) + steps;
            queue.push((Reverse(score), Reverse(steps), dx, dy));
        }
    }

    unreachable!()
}

fn manhattan((ax, ay): (usize, usize), (bx, by): (usize, usize)) -> usize {
    ax.abs_diff(bx) + ay.abs_diff(by)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    #[test]
    fn it_solves_challenge_1() {
        assert_eq!(challenge_1(TEST_INPUT), 31);
    }

    #[test]
    fn it_solves_challenge_2() {
        assert_eq!(challenge_2(TEST_INPUT), 29);
    }
}

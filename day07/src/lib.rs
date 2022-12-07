pub fn challenge_1(input: &str) -> usize {
    let (_, dir_sizes) = parse_dir_sizes(input);

    const MAX_SIZE: usize = 100_000;

    dir_sizes.into_iter().filter(|&size| size < MAX_SIZE).sum()
}

pub fn challenge_2(input: &str) -> usize {
    let (root_size, dir_sizes) = parse_dir_sizes(input);

    const DISK_SIZE: usize = 70_000_000;
    const UNUSED_SIZE: usize = 30_000_000;
    let required = UNUSED_SIZE - (DISK_SIZE - root_size);

    dir_sizes
        .into_iter()
        .filter(|&size| size >= required)
        .min()
        .expect("no directories are the required size")
}

fn parse_dir_sizes(input: &str) -> (usize, Vec<usize>) {
    let mut dir_stack = Vec::new();
    let mut dir_sizes = Vec::new();

    for line in input.lines() {
        if let Some(line) = line.strip_prefix("$ cd ") {
            if line == ".." {
                let size = dir_stack.pop().expect("empty stack");
                dir_sizes.push(size);
                *dir_stack.last_mut().expect("no parent") += size;
            } else {
                dir_stack.push(0);
            }
        } else if let Some((size, _filename)) = line.split_once(' ') {
            if let Ok(size) = size.parse::<usize>() {
                *dir_stack.last_mut().expect("empty stack") += size;
            }
        }
    }

    let mut root_size = 0;
    for size in dir_stack.into_iter().rev() {
        root_size += size;
        dir_sizes.push(root_size);
    }

    (root_size, dir_sizes)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn it_solves_challenge_1() {
        assert_eq!(challenge_1(TEST_INPUT), 95437);
    }

    #[test]
    fn it_solves_challenge_2() {
        assert_eq!(challenge_2(TEST_INPUT), 24933642);
    }
}

use std::str::Lines;

pub fn challenge_1(input: &str) -> usize {
    let root = read_dirs(&mut input.lines());

    const MAX_SIZE: usize = 100_000;

    get_dir_sizes(&root).filter(|&size| size < MAX_SIZE).sum()
}

pub fn challenge_2(input: &str) -> usize {
    let root = read_dirs(&mut input.lines());

    const DISK_SIZE: usize = 70_000_000;
    const UNUSED_SIZE: usize = 30_000_000;
    let required = UNUSED_SIZE - (DISK_SIZE - root.size);

    get_dir_sizes(&root)
        .filter(|&size| size >= required)
        .min()
        .expect("no directories are the required size")
}

fn read_dirs(lines: &mut Lines<'_>) -> Dir {
    let mut dir = Dir {
        size: 0,
        entries: Vec::new(),
    };

    while let Some(line) = lines.next() {
        // Ignore cd root and ls command
        if line == "$ cd /" || line == "$ ls" {
            continue;
        }

        // Ignore dir names
        if line.starts_with("dir") {
            continue;
        }

        // Stop reading lines if backtracking
        if line == "$ cd .." {
            break;
        }

        // Recurse into a directory
        if line.starts_with("$ cd") {
            let child = read_dirs(lines);
            dir.size += child.size;
            dir.entries.push(child);
        } else {
            // Parse a file size and add to current directory size
            dir.size += line
                .split_once(' ')
                .expect("malformed file entry")
                .0
                .parse::<usize>()
                .expect("malformed file size");
        }
    }

    dir
}

// Map all entires into their sizes
fn get_dir_sizes(root: &Dir) -> impl Iterator<Item = usize> + '_ {
    recurse_dirs(root).map(|dir| dir.size)
}

// Recursively chain all entries into an iterator
fn recurse_dirs(root: &Dir) -> Box<dyn Iterator<Item = &Dir> + '_> {
    Box::new(std::iter::once(root).chain(root.entries.iter().flat_map(recurse_dirs)))
}

#[derive(Debug)]
struct Dir {
    size: usize,
    entries: Vec<Dir>,
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

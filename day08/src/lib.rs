use std::borrow::BorrowMut;

pub fn challenge_1(input: &str) -> usize {
    let trees = parse_trees(input);

    let mut visible_trees = vec![vec![false; trees[0].len()]; trees.len()];

    // Left to right, top to bottom
    let mut highest_y = vec![None; trees.len()];
    for (i, tree_row) in trees.iter().enumerate() {
        let mut highest_x = None;
        for (j, tree) in tree_row.iter().enumerate() {
            if let Some(highest_y) = highest_y[j].borrow_mut() {
                if *highest_y < tree {
                    visible_trees[i][j] = true;
                    *highest_y = tree;
                }
            } else {
                visible_trees[i][j] = true;
                highest_y[j] = Some(tree);
            }

            if let Some(highest_x) = highest_x.borrow_mut() {
                if *highest_x < tree {
                    visible_trees[i][j] = true;
                    *highest_x = tree;
                }
            } else {
                visible_trees[i][j] = true;
                highest_x = Some(tree);
            }
        }
    }

    // Right to left, bottom to top
    let mut highest_y = vec![None; trees.len()];
    for (i, tree_row) in trees.iter().enumerate().rev() {
        let mut highest_x = None;
        for (j, tree) in tree_row.iter().enumerate().rev() {
            if let Some(highest_y) = highest_y[j].borrow_mut() {
                if *highest_y < tree {
                    visible_trees[i][j] = true;
                    *highest_y = tree;
                }
            } else {
                visible_trees[i][j] = true;
                highest_y[j] = Some(tree);
            }

            if let Some(highest_x) = highest_x.borrow_mut() {
                if *highest_x < tree {
                    visible_trees[i][j] = true;
                    *highest_x = tree;
                }
            } else {
                visible_trees[i][j] = true;
                highest_x = Some(tree);
            }
        }
    }

    visible_trees
        .into_iter()
        .flat_map(|row| row.into_iter().map(|visible| if visible { 1 } else { 0 }))
        .sum()
}

pub fn challenge_2(input: &str) -> usize {
    let trees = parse_trees(input);
    let mut max_score = 0;

    for x in 0..trees.len() {
        for y in 0..trees[x].len() {
            let score = get_score(&trees, x, y);

            if max_score < score {
                max_score = score;
            }
        }
    }

    max_score
}

fn parse_trees(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn get_score(trees: &Vec<Vec<u32>>, x: usize, y: usize) -> usize {
    let current_height = trees[x][y];
    let mut left = 0;
    let mut right = 0;
    let mut up = 0;
    let mut down = 0;

    // look left from point
    for x in (0..x).rev() {
        left += 1;
        if trees[x][y] >= current_height {
            break;
        }
    }

    // look right from point
    for x in x + 1..trees.len() {
        right += 1;
        if trees[x][y] >= current_height {
            break;
        }
    }

    // look up from point
    for y in (0..y).rev() {
        up += 1;
        if trees[x][y] >= current_height {
            break;
        }
    }

    // look down from point
    for y in y + 1..trees[x].len() {
        down += 1;
        if trees[x][y] >= current_height {
            break;
        }
    }

    left * right * up * down
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn it_solves_challenge_1() {
        assert_eq!(challenge_1(TEST_INPUT), 21);
    }

    #[test]
    fn it_solves_challenge_2() {
        assert_eq!(challenge_2(TEST_INPUT), 8);
    }
}

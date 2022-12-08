use ndarray::{s, Array, Array2};
use std::cmp::max;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    // Parse into ndarray
    let lines = input.lines();
    let line_len = lines.count();
    let mut trees_map = Array::<u32, _>::zeros((line_len, line_len));

    for (index, line) in input.lines().enumerate() {
        let numbers: Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();
        trees_map.row_mut(index).assign(&Array::from(numbers));
    }

    // Iterate trees to see if visible from outside
    // Add all outside trees
    let mut visible_trees = line_len * 4 - 4;

    let interior_view = trees_map.slice(s!(1..line_len - 1, 1..line_len - 1));
    for (coord, height) in interior_view.indexed_iter() {
        // Adjust coord to outer array
        let modified_coord = (coord.0 + 1, coord.1 + 1);

        // Check if tree is visible from outside - all trees up, down, left, right are smaller
        if check_visible(*height, &modified_coord, &trees_map) {
            visible_trees += 1;
        }
    }

    println!("Part 1: {visible_trees}");

    let mut highest_scenic_score = 0;
    for (coord, height) in interior_view.indexed_iter() {
        // Adjust coord to outer array
        let modified_coord = (coord.0 + 1, coord.1 + 1);
        let scenic_score = get_scenic_score(*height, &modified_coord, &trees_map);
        highest_scenic_score = max(highest_scenic_score, scenic_score);
    }

    println!("Part 2: {highest_scenic_score}");
}

fn get_scenic_score(height: u32, coord: &(usize, usize), trees_map: &Array2<u32>) -> usize {
    let right = s!(coord.0, coord.1 + 1..trees_map.ncols());
    // Make sure to reverse left and up
    let left = s!(coord.0, 0..coord.1; -1);
    let up = s!(0..coord.0, coord.1);
    let down = s!(coord.0 + 1..trees_map.nrows(), coord.1);

    let mut scenic_score = 1;
    for slice in [right, down, left] {
        let array_view = trees_map.slice(slice);
        let mut score = 0;
        for tree_height in array_view.iter() {
            score += 1;
            if *tree_height >= height {
                break;
            }
        }
        scenic_score *= score;
    }

    // Reverse order up
    // Todo: figure out how to reverse a 2d array and include in loop above
    let array_view = trees_map.slice(up);
    let mut score = 0;
    let mut reversed: Vec<&u32> = array_view.iter().collect();
    reversed.reverse();

    for tree_height in reversed.iter() {
        score += 1;
        if *tree_height >= &height {
            break;
        }
    }
    scenic_score *= score;

    scenic_score
}

fn check_visible(height: u32, coord: &(usize, usize), trees_map: &Array2<u32>) -> bool {
    let right = s!(coord.0..=coord.0, coord.1 + 1..trees_map.ncols());
    let left = s!(coord.0..=coord.0, 0..coord.1);
    let down = s!(0..coord.0, coord.1..=coord.1);
    let up = s!(coord.0 + 1..trees_map.nrows(), coord.1..=coord.1);
    for slice in [right, left, down, up] {
        let array_view = trees_map.slice(slice);
        if array_view.iter().all(|x| *x < height) {
            return true;
        }
    }

    false
}

// Advent of Code Day 08 part 1
// This is still quite sloppy. I can think of some quick improvements (I think
// I only need one grid) and there's probably an easier way to do the up/down/left/right
// parses. But I've been lazy and unmotivated recently so I'll commit part 1 and come
// back to part 2 at some point, I hope.

use std::fs;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum TreeStatus {
    Visible,
    NotVisible,
}

fn main() {
    let input_file = fs::read_to_string("input.txt")
        .expect("The filename is probably incorrect.");

    let tree_map: Vec<&str> = input_file.split_ascii_whitespace().collect();

    let mut tree_grid: HashMap<(usize, usize), u8> = HashMap::new();
    let mut visible_grid: HashMap<(usize, usize), TreeStatus> = HashMap::new();
    // note that the indexes will run up to size - 1
    let grid_size = tree_map.get(0).unwrap_or(&"").len();

    for (y, line) in tree_map.iter().enumerate() {
        for (x, height) in line.chars().enumerate() {
            tree_grid.insert((x, y), height.to_digit(10).unwrap_or(10) as u8);
            visible_grid.insert((x, y), TreeStatus::NotVisible);
        }
    }

    // all trees on the edge are visible
    for (_, tree_status) in visible_grid.iter_mut().filter(|((x, y), _)| *x == 0 || *y == 0 || *x == grid_size - 1 || *y == grid_size - 1) {
        *tree_status = TreeStatus::Visible
    }

    // run down every line in four directions
    // if the tree height is greater than all other previously encountered trees, it's visible
    // left
    for y in 1..grid_size {
        let mut current_max = *tree_grid.get(&(0, y)).unwrap_or(&100);
        for x in 1..grid_size {
            if let (Some(current_tree_height), Some(current_tree_visibility)) = (tree_grid.get(&(x, y)), visible_grid.get(&(x, y))) {
                if current_tree_height > &current_max {
                    current_max = *current_tree_height;

                    if *current_tree_visibility == TreeStatus::NotVisible {
                        visible_grid.insert((x, y), TreeStatus::Visible);
                    }
                }
            }
        }
    }

    // right
    for y in 1..grid_size {
        let mut current_max = *tree_grid.get(&(grid_size - 1, y)).unwrap_or(&100);
        for x in (1..grid_size).rev() {
            if let (Some(current_tree_height), Some(current_tree_visibility)) = (tree_grid.get(&(x, y)), visible_grid.get(&(x, y))) {
                if current_tree_height > &current_max {
                    current_max = *current_tree_height;

                    if *current_tree_visibility == TreeStatus::NotVisible {
                        visible_grid.insert((x, y), TreeStatus::Visible);
                    }
                }
            }
        }
    }

    // down
    for x in 1..grid_size {
        let mut current_max = *tree_grid.get(&(x, 0)).unwrap_or(&100);
        for y in 1..grid_size {
            if let (Some(current_tree_height), Some(current_tree_visibility)) = (tree_grid.get(&(x, y)), visible_grid.get(&(x, y))) {
                if current_tree_height > &current_max {
                    current_max = *current_tree_height;

                    if *current_tree_visibility == TreeStatus::NotVisible {
                        visible_grid.insert((x, y), TreeStatus::Visible);
                    }
                }
            }
        }
    }

    // up
    for x in 1..grid_size {
        let mut current_max = *tree_grid.get(&(x, grid_size - 1)).unwrap_or(&100);
        for y in (1..grid_size).rev() {
            if let (Some(current_tree_height), Some(current_tree_visibility)) = (tree_grid.get(&(x, y)), visible_grid.get(&(x, y))) {
                if current_tree_height > &current_max {
                    current_max = *current_tree_height;

                    if *current_tree_visibility == TreeStatus::NotVisible {
                        visible_grid.insert((x, y), TreeStatus::Visible);
                    }
                }
            }
        }
    }

    println!("There are {} visible trees.", visible_grid.values().filter(|status| **status == TreeStatus::Visible).count());
}

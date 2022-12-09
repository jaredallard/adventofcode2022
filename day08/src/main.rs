use std::{collections::HashMap, fs};

fn get_tree_info(grid: &Vec<Vec<u8>>, x: usize, y: usize) -> (bool, usize) {
    let mut above_vis = true;
    let mut below_vis = true;
    let mut left_vis = true;
    let mut right_vis = true;
    let mut always_vis = false;

    let mut up = 0;
    let mut down = 0;
    let mut left = 0;
    let mut right = 0;

    // always visible if we're on the edge
    if x == 0 || x == grid[y].len() - 1 || y == 0 || y == grid.len() - 1 {
        always_vis = true;
    }
    let our_height = grid[y][x];

    // check above
    for i in (0..y).rev() {
        up += 1;
        if our_height <= grid[i][x] {
            above_vis = false;
            break;
        }
    }

    // check below
    for i in y + 1..grid.len() {
        down += 1;
        if our_height <= grid[i][x] {
            below_vis = false;
            break;
        }
    }

    // check left
    for i in (0..x).rev() {
        left += 1;
        if our_height <= grid[y][i] {
            left_vis = false;
            break;
        }
    }

    // check right
    for i in x + 1..grid[y].len() {
        right += 1;
        if our_height <= grid[y][i] {
            right_vis = false;
            break;
        }
    }

    let score = up * down * left * right;
    return (
        above_vis || below_vis || left_vis || right_vis || always_vis,
        score,
    );
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("failed to read file");

    let mut grid: Vec<Vec<u8>> = Vec::new();
    let mut visible: HashMap<String, bool> = HashMap::new();
    for line in contents.lines() {
        let mut row: Vec<u8> = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).expect("not a digit") as u8);
        }
        grid.push(row);
    }

    let mut processed_trees = 0;
    let mut highest_score: usize = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            // println!("Tree at ({}, {})", x, y);
            processed_trees += 1;

            let key = format!("{},{}", x, y);
            let (is_visible, score) = get_tree_info(&grid, x, y);
            if is_visible {
                visible.insert(key, true);
                print!("X")
            } else {
                print!(" ")
            }

            if score > highest_score {
                highest_score = score;
            }
        }
        println!();
    }

    println!("Processed {} trees", processed_trees);
    println!("");
    println!("Problem 1: {}", visible.len());
    println!("Problem 2: {}", highest_score);
}

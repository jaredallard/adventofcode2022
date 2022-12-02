// Rock:
//  Points: 1
//  Symbols: A, X
//
// Paper:
//  Points: 2
//  Symbols: B, Y
//
// Scissors:
//  Points: 3
//  Symbols: C, Z

// Results:
// - Loss: 0pts
// - Draw: 3pts
// - Win: 6pts

use std::collections::HashMap;
use std::fs;
use std::io::{self, prelude::*};

const WIN_POINTS: u32 = 6;
const DRAW_POINTS: u32 = 3;
const LOSS_POINTS: u32 = 0;

fn result(them: &str, us: &str) -> u32 {
    match (them, us) {
        // Win
        ("A", "Y") | ("B", "Z") | ("C", "X") => WIN_POINTS,
        // Draw
        ("A", "X") | ("B", "Y") | ("C", "Z") => DRAW_POINTS,
        // Loss
        ("A", "Z") | ("B", "X") | ("C", "Y") | _ => LOSS_POINTS,
    }
}

fn main() {
    // open the file and create a reader
    let f = fs::File::open("input.txt").expect("failed to open file");
    let reader = io::BufReader::new(f);

    // hashmap of the moves to their points
    let mut moves_to_points = HashMap::new();
    moves_to_points.insert("X", 1);
    moves_to_points.insert("Y", 2);
    moves_to_points.insert("Z", 3);

    // map of instructions to win/loss/draw points that we need to
    // calculate
    let mut symbols_to_win_points = HashMap::new();
    symbols_to_win_points.insert("X", LOSS_POINTS);
    symbols_to_win_points.insert("Y", DRAW_POINTS);
    symbols_to_win_points.insert("Z", WIN_POINTS);

    let mut problem_one_points: u32 = 0;
    let mut problem_two_points: u32 = 0;
    for line in reader.lines() {
        // read the line
        let line = line.expect("failed to read line");

        // split the line into two parts
        let mut parts = line.split_whitespace();

        // read the rock paper scissors moves
        let them = parts.next().expect("failed to read first part");
        let us = parts.next().expect("failed to read second part");

        // calculate the result
        let res = result(them, us);

        let p2_desired_result = symbols_to_win_points
            .get(us)
            .expect("failed to get desired result");

        // find the move that would have resulted in the desired result
        // and add it to the second problem's points total
        for i in ["X", "Y", "Z"] {
            let res = result(them, i);
            if res == *p2_desired_result {
                problem_two_points += moves_to_points[i] + res;
                break;
            }
        }

        let score = res + moves_to_points[us];
        problem_one_points += score;
    }
    println!("Problem 1: {}", problem_one_points);
    println!("Problem 2: {}", problem_two_points);
}

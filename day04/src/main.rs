use std::fs;
use std::io::{self, prelude::*};

// returns true if any of the ranges provided fully include
// the other
fn problem_1(parts: &Vec<Vec<u32>>) -> bool {
    let first_range = parts.first().expect("failed to get first index");
    let second_range = parts.get(1).expect("failed to get second index");

    // check if the first range contains the second range of ints
    let first_range_start = first_range
        .first()
        .expect("failed to get first range's first value");
    let first_range_end = first_range
        .last()
        .expect("failed to get first range's second value");
    let second_range_start = second_range
        .first()
        .expect("failed to get first range's first value");
    let second_range_end = second_range
        .last()
        .expect("failed to get first range's second value");

    // check if the first range includes the second or the second contains the first
    return (first_range_start.le(second_range_start) && second_range_end.le(first_range_end))
        || (second_range_start.le(first_range_start) && first_range_end.le(second_range_end));
}

// returns true if any of the ranges overlap at all
fn problem_2(parts: &Vec<Vec<u32>>) -> bool {
    let first_range = parts.first().expect("failed to get first index");
    let second_range = parts.get(1).expect("failed to get second index");

    // check if the first range contains the second range of ints
    let first_range_start = first_range
        .first()
        .expect("failed to get first range's first value");
    let first_range_end = first_range
        .last()
        .expect("failed to get first range's second value");
    let second_range_start = second_range
        .first()
        .expect("failed to get first range's first value");
    let second_range_end = second_range
        .last()
        .expect("failed to get first range's second value");

    // check if any of the ranges overlap at all (even if not fully)
    return (first_range_start.le(second_range_start) && second_range_start.le(first_range_end))
        || (second_range_start.le(first_range_start) && first_range_start.le(second_range_end));
}

fn main() {
    let file = fs::File::open("input.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut p1_total: u32 = 0;
    let mut p2_total: u32 = 0;
    for line in reader.lines() {
        // format int-int,int-int
        let line = line.expect("failed to read line");

        // Split on "," then split again on "-" and map
        // the values into <Vec<Vec<u32>>>
        let parts = line
            .split(",")
            .map(|s| {
                // map - into [0,0]
                s.split("-")
                    .map(|s| s.parse::<u32>().expect("cant turn into int"))
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();
        if !parts.len().eq(&2) {
            println!("Invalid input: {}", line);
            std::process::exit(1)
        }

        if problem_1(&parts) {
            p1_total += 1;
        }

        if problem_2(&parts) {
            p2_total += 1;
        }
    }

    println!("Problem 1: {}", p1_total);
    println!("Problem 2: {}", p2_total);
}

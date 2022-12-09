use std::collections::BTreeSet;
use std::fs;
use std::str::Lines;

const LEFT: &str = "L";
const RIGHT: &str = "R";
const UP: &str = "U";
const DOWN: &str = "D";

fn sim(lines: Lines, num_knots: usize) -> usize {
    let mut knots = vec![];

    // generate the input knots
    for _ in 0..num_knots {
        knots.push((0, 0));
    }

    // track all visited points
    let mut visited: BTreeSet<(i32, i32)> = BTreeSet::new();
    visited.insert(*knots.last().expect("no knots?"));

    // simulate the movement
    for line in lines {
        let parts: Vec<&str> = line.split(|c| c == ' ').collect();
        let direction = *parts.first().expect("Unable to get direction");
        let count = *parts.last().expect("Unable to get count");
        let total = count.parse().expect("Unable to parse number");

        // move through
        for _ in 0..total {
            // calculate grid movement
            match direction {
                LEFT => knots[0].0 -= 1,
                RIGHT => knots[0].0 += 1,
                UP => knots[0].1 += 1,
                DOWN => knots[0].1 -= 1,
                _ => panic!("????"),
            }

            // move the knots
            for k in 1..num_knots {
                if !((knots[k - 1].0 - knots[k].0).abs() >= 2
                    || (knots[k - 1].1 - knots[k].1).abs() >= 2)
                {
                    continue;
                }

                // move based on the previous knot
                if knots[k].0 < knots[k - 1].0 {
                    knots[k].0 += 1;
                } else if knots[k].0 > knots[k - 1].0 {
                    knots[k].0 -= 1;
                }

                if knots[k].1 < knots[k - 1].1 {
                    knots[k].1 += 1;
                } else if knots[k].1 > knots[k - 1].1 {
                    knots[k].1 -= 1;
                }
            }

            // track the tail knot's position
            visited.insert(*knots.last().expect("no knots"));
        }
    }

    visited.len()
}

fn main() {
    let content = fs::read_to_string("inputs/day09.txt").expect("Unable to read file");
    let lines = content.lines();

    let problem_1 = sim(lines.clone(), 2);
    let problem_2 = sim(lines, 10);

    println!("Problem 1: {}", problem_1);
    println!("Problem 2: {}", problem_2);
}

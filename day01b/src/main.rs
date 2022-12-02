use std::fs;
use std::io::{self, prelude::*};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Elf {
    calories: i32,
}

fn main() {
    // open the file and create a reader
    let f = fs::File::open("input.txt").expect("failed to open file");
    let reader = io::BufReader::new(f);

    let mut elves: Vec<Elf> = Vec::new();

    let mut current_calories = 0;

    for l in reader.lines() {
        let line = l.expect("failed to read line");

        // if the line is empty, we have a new elf
        if line == "" {
            elves.push(Elf {
                calories: current_calories,
            });
            current_calories = 0;
            continue;
        }

        let calories = line.parse::<i32>().expect("failed to parse line");
        current_calories += calories;
    }

    // sort by calories
    elves.sort_by(|a, b| b.calories.cmp(&a.calories));

    // add together the first 3
    let mut total = 0;
    for i in 0..3 {
        total += elves[i].calories;
    }
    println!("total: {}", total);
}

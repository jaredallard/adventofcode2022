use std::fs;
use std::io::{self, prelude::*};

fn main() {
  // open the file and create a reader
  let f = fs::File::open("input.txt").expect("failed to open file");
  let reader = io::BufReader::new(f);

  // The "most calories" elf
  let mut elf_with_most = 1;
  let mut elf_with_most_calories = 0;

  // track the calorie count across iterations
  let mut current_elf = 1;
  let mut current_elf_calories = 0;


  for l in reader.lines() {
    let line = l.expect("failed to read line");

    if line == "" {
      println!("Elf {} had {} calories", current_elf, current_elf_calories);
      
      // check if the elf we were on had more calories than
      // our current best
      if current_elf_calories> elf_with_most_calories {
        println!("Elf {} became new most ({} > {})", current_elf, current_elf_calories, elf_with_most_calories);
        elf_with_most_calories = current_elf_calories;
        elf_with_most = current_elf;
      }

      // reset the current elf stats
      current_elf_calories = 0;
      current_elf += 1;

      continue;
    }

    // add the calories for this line
    current_elf_calories+=line.parse::<i32>().unwrap();
  }

  println!("Elf {} had the most calories: {}", elf_with_most, elf_with_most_calories);
}
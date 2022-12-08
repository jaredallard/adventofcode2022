use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    // split the contents into lines
    let lines: Vec<&str> = contents.split_whitespace().collect();

    // iterate over the lines
    let mut processed_chars = 0;
    for line in lines {
        // split by unicode character
        let chars: Vec<char> = line.chars().collect();

        // hashmap of each character
        let mut keys: Vec<char> = Vec::new();

        // iterate over the characters
        for c in chars {
            processed_chars += 1;

            // if we're not at 4 characters, just insert
            if keys.len() < 14 {
                keys.push(c);
                continue;
            }

            // check if all 4 characters were unique
            let mut unique = true;
            for i in 0..keys.len() {
                for j in 0..keys.len() {
                    if i == j {
                        continue;
                    }
                    if keys[i] == keys[j] {
                        unique = false;
                        break;
                    }
                }
            }
            // not unique, pop it on and remove the first character
            if !unique {
                keys.remove(0);
                keys.push(c);
            }

            // unique, print it out
            if unique {
                for k in keys.iter() {
                    print!("{}", k);
                }
                println!(": {}", processed_chars - 1);
                break;
            }
        }
    }
}

use std::fs;

const ROWS: usize = 9;

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let mut stacks: Vec<Vec<char>> = vec![vec![]; ROWS];
    let mut lines = input.lines();

    // gather the initial stacks
    // supposedly Some is safer than expect? unclear
    while let Some(line) = lines.next() {
        // skip the header
        if line.starts_with(" 1 ") {
            println!("read initial stacks");
            break;
        }
        for i in 0..ROWS {
            // why isn't this just called get? :|
            let docker_container = line.chars().nth((4 * i) + 1).unwrap();
            if docker_container.is_ascii_uppercase() {
                stacks[i].insert(0, docker_container);
            }
        }
    }

    // skip the empty line for the moves
    let nextline = lines.next().unwrap().trim();
    if !nextline.is_empty() {
        panic!("expected empty line, got {}", nextline);
    }

    let mut moves = Vec::new();
    while let Some(line) = lines.next() {
        // split on whitespace
        let txt: Vec<&str> = line.split_ascii_whitespace().collect();

        // get the container
        let docker_container: usize = txt[1].parse().unwrap();

        // store the movement instructions
        let from_x: usize = txt[3].parse::<usize>().unwrap() - 1;
        let to_x: usize = txt[5].parse::<usize>().unwrap() - 1;

        moves.push((docker_container, from_x, to_x))
    }
    return (stacks, moves);
}

fn problem_a(input: &str) -> String {
    let (mut stacks, moves) = parse(input);
    for (docker_container, from_i, to_i) in moves {
        // shippin code
        for _ in 0..docker_container {
            let x = stacks[from_i].pop().unwrap();
            stacks[to_i].push(x);
        }
    }
    stacks.iter().map(|s| s.last().unwrap()).collect()
}

// is it over yet
fn problem_b(input: &str) -> String {
    let (mut stacks, moves) = parse(input);
    for (docker_container, from_i, to_i) in moves {
        let j = stacks[from_i].len() - docker_container;
        let mut x = stacks[from_i].split_off(j);
        stacks[to_i].append(&mut x)
    }
    stacks.iter().map(|s| s.last().unwrap()).collect()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Problem 1 {}", problem_a(&input));
    println!("Problem 2 {}", problem_b(&input));
}

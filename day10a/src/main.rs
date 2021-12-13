use std::io::{self, prelude::*, BufReader};

fn main() {
    let lines: Vec<_> = BufReader::new(io::stdin()).lines()
    .map(|l| l.unwrap())
    .collect();

    let mut score = 0;
    let mut stack = vec![];
    for line in lines {
        let mut bad_char = '\0';
        for c in line.chars() {
            if is_start(c) {
                stack.push(expected(c));
            } else {
                match stack.pop() {
                    Some(x) => {
                        if c != x {
                            bad_char = c;
                            break;
                        }
                    },
                    None => {
                        println!("Closing char on empty stack: {}", c);
                        bad_char = c;
                        break;
                    }
                }
            }
        }
        if bad_char != '\0' {
            score += score_for(bad_char);
        }
    }

    println!("{}", score);
}

fn is_start(c: char) -> bool {
    match c {
        '(' | '[' | '{' | '<' => true,
        _ => false
    }
}

fn expected(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => {
            panic!("Unexpected char: {}", c)
        }
    }
}

fn score_for(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Can't score {}", c)
    }
}
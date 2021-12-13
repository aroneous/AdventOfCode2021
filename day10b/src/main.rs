use std::io::{self, prelude::*, BufReader};

fn main() {
    let lines: Vec<_> = BufReader::new(io::stdin()).lines()
    .map(|l| l.unwrap())
    .collect();

    let mut scores = vec![];
    for line in lines {
        let mut stack = vec![];
        let mut corrupt = false;
        for c in line.chars() {
            if is_start(c) {
                stack.push(expected(c));
            } else {
                match stack.pop() {
                    Some(x) => {
                        if c != x {
                            corrupt = true;
                            break;
                        }
                    },
                    None => {
                        println!("Closing char on empty stack: {}", c);
                        corrupt = true;
                        break;
                    }
                }
            }
        }
        if !corrupt {
            let mut score = 0;
            loop {
                match stack.pop() {
                    Some(c) => {
                        score = score * 5 + score_for(c);
                    },
                    None => {
                        break;
                    }
                }
            }
            // println!("{}", score);
            scores.push(score);
        }
    }

    scores.sort();
    println!("{}", scores[scores.len()/2]);
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

fn score_for(c: char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("Can't score {}", c)
    }
}
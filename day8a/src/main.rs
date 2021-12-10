use std::io::{self, prelude::*, BufReader};

fn main() {
    let lines: Vec<_> = BufReader::new(io::stdin()).lines()
    .map(|l| l.unwrap())
    .collect();

    let mut count = 0;
    for line in lines {
        let parts: Vec<_> = line.split("|").collect();
        let outputs: Vec<_> = parts[1].trim().split_whitespace()
        .collect();

        count += outputs.iter()
        .filter(|s| {
            match s.len() {
                2 | 4 | 3 | 7 => true,
                _ => false
            }
        }).count();
    }

    println!("{}", count);
}
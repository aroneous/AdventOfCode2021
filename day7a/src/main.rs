use std::io::{self, prelude::*, BufReader};

fn main() {
    let lines: Vec<_> = BufReader::new(io::stdin()).lines()
    .map(|l| l.unwrap())
    .collect();

    let crabs: Vec<_> = lines[0].trim().split(",")
    .map(|c| c.parse::<usize>().unwrap())
    .collect();

    let highest = *crabs.iter().max().unwrap();

    let mut counts: Vec<i32> = vec![0; highest + 1];
    for f in crabs {
        counts[f] += 1;
    }

    // let mut least_pos = 0;
    let mut least_fuel = -1;

    for pos in 0..counts.len() {
        let mut fuel:i32 = 0;
        for idx in 0..counts.len() {
            let offset = (idx as i32 - pos as i32).abs();
            fuel += offset * counts[idx];
        }
        if least_fuel < 0 || fuel < least_fuel {
            least_fuel = fuel;
            // least_pos = pos;
        }
    }

    println!("{}", least_fuel);
}
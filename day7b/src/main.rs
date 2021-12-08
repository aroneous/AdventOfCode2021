use std::io::{self, prelude::*, BufReader};

fn main() {
    let lines: Vec<_> = BufReader::new(io::stdin()).lines()
    .map(|l| l.unwrap())
    .collect();

    let crabs: Vec<_> = lines[0].trim().split(",")
    .map(|c| c.parse::<usize>().unwrap())
    .collect();

    let highest = *crabs.iter().max().unwrap();

    let mut counts: Vec<i64> = vec![0; highest + 1];
    for f in crabs {
        counts[f] += 1;
    }

    // let mut least_pos = 0;
    let mut least_fuel = -1;

    // println!("Computing costs");

    let mut costs:Vec<i64> = vec![0; highest + 1];
    for j in 0..costs.len() {
        for k in j..costs.len() {
            costs[k] += j as i64;
        }
    }

    // println!("Costs: {:?}", costs);

    for pos in 0..counts.len() {
        let mut fuel:i64 = 0;
        for idx in 0..counts.len() {
            let offset = (idx as i64 - pos as i64).abs() as usize;
            fuel += costs[offset] * counts[idx];
        }
        if least_fuel < 0 || fuel < least_fuel {
            least_fuel = fuel;
            // least_pos = pos;
        }
    }

    println!("{}", least_fuel);
}
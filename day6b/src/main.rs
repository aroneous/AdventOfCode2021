use std::io::{self, prelude::*, BufReader};

fn main() {
    let lines: Vec<_> = BufReader::new(io::stdin()).lines()
    .map(|l| l.unwrap())
    .collect();

    let fish: Vec<_> = lines[0].trim().split(",")
    .map(|c| c.parse::<usize>().unwrap())
    .collect();

    let mut counts: Vec<u128> = vec![0; 9];
    for f in fish {
        counts[f] += 1;
    }

    for _ in 0..256 {
        do_generation(&mut counts);
    }

    println!("{}", counts.iter().sum::<u128>());

    // let num_new = fish.iter().filter(|n| *n == &0).count();
    // fish = fish.iter().map(|n| if *n == 0 { 8 } else { *n - 1 }).collect();
    // for _ in 0..num_new {
    //     fish.push(8);
    // }
}

fn do_generation(counts: &mut Vec<u128>) {
    let new = counts[0];
    for idx in 0..counts.len()-1 {
        counts[idx] = counts[idx+1];
    }
    counts[6] += new;
    counts[8] = new;
}
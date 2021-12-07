use std::io::{self, prelude::*, BufReader};

fn main() {
    let lines: Vec<_> = BufReader::new(io::stdin()).lines()
    .map(|l| l.unwrap())
    .collect();

    let mut fish: Vec<_> = lines[0].trim().split(",")
    .map(|c| c.parse::<i32>().unwrap())
    .collect();

    for _ in 0..80 {
        fish = do_generation(&fish);
    }

    println!("{}", fish.len());

    // let num_new = fish.iter().filter(|n| *n == &0).count();
    // fish = fish.iter().map(|n| if *n == 0 { 8 } else { *n - 1 }).collect();
    // for _ in 0..num_new {
    //     fish.push(8);
    // }
}

fn do_generation(fish: &Vec<i32>) -> Vec<i32> {
    let num_new = fish.iter().filter(|n| *n == &0).count();
    let mut new_fish: Vec<_> = fish.iter().map(|n| if *n == 0 { 6 } else { *n - 1 }).collect();
    for _ in 0..num_new {
        new_fish.push(8);
    }
    new_fish
}
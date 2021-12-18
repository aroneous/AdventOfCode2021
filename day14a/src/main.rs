use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let re = Regex::new(r"(.)(.) -> (.)").unwrap();

    let lines: Vec<_> = BufReader::new(io::stdin()).lines()
    .map(|l| l.unwrap())
    .collect();

    let mut seq: Vec<char> = lines[0].chars().collect();

    let mut rules = vec![];
    for line in &lines[2..] {
        let cap = re.captures(line).unwrap();
        let rule = (cap[1].chars().next().unwrap(), cap[2].chars().next().unwrap(),
                 cap[3].chars().next().unwrap());
        rules.push(rule);
    }

    // println!("{:?}", seq);
    // println!("{:?}", rules);

    for _ in 0..10 {
        let mut idx = 0;
        while idx < seq.len() - 1 {
            // println!("idx: {}", idx);
            for (a, b, ins) in &rules {
                if seq[idx] == *a && seq[idx + 1] == *b {
                    idx += 1;
                    // println!("Incremented idx");
                    seq.insert(idx, *ins);
                    break;
                }
            }
            idx += 1;
        }
    }

    let mut counts = HashMap::<char, usize>::new();

    for c in &seq {
        counts.entry(*c)
            .and_modify(|e| { *e += 1 })
            .or_insert(1);
    }

    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();
    println!("{}", max - min);
}
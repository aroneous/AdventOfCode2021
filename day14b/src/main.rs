use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let re = Regex::new(r"(.)(.) -> (.)").unwrap();

    let lines: Vec<_> = BufReader::new(io::stdin()).lines()
    .map(|l| l.unwrap())
    .collect();

    let seq: Vec<char> = lines[0].chars().collect();

    let mut rules = vec![];
    for line in &lines[2..] {
        let cap = re.captures(line).unwrap();
        let rule = (cap[1].chars().next().unwrap(), cap[2].chars().next().unwrap(),
                 cap[3].chars().next().unwrap());
        rules.push(rule);
    }

    let mut counts = HashMap::<char, i64>::new();
    for c in &seq {
        counts.entry(*c)
        .and_modify(|e| { *e += 1 })
        .or_insert(1);
    }

    let mut table = HashMap::<(char, char), i64>::new();
    for (a, b, _ins) in &rules {
        table.insert((*a, *b), 0);
    }
    for idx in 0..seq.len() - 1 {
        match table.get_mut(&(seq[idx], seq[idx+1])) {
            Some(c) => {
                *c += 1;
            },
            None => {}
        }
    }

    for _ in 0..40 {
        let mut productions = HashMap::<(char, char), i64>::new();
        for (a, b, ins) in &rules {
            match table.get_mut(&(*a, *b)) {
                Some(c) => {
                    counts.entry(*ins)
                    .and_modify(|e| { *e += *c; })
                    .or_insert(*c);
                    productions.entry((*a, *b))
                    .and_modify(|e| { *e -= *c; })
                    .or_insert(-*c);
                    productions.entry((*a, *ins))
                    .and_modify(|e| { *e += *c; })
                    .or_insert(*c);
                    productions.entry((*ins, *b))
                    .and_modify(|e| { *e += *c; })
                    .or_insert(*c);
                },
                None => {}
            }
        }
        for (k, v) in &mut table {
            match productions.get(k) {
                Some(c) => {
                    *v += c;
                },
                None => {}
            }
        }
    }

    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();
    println!("{}", max - min);
}
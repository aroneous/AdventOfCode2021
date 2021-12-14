use std::io::{self, prelude::*, BufReader};
use std::collections::{HashMap, HashSet};
use std::string::String;

fn main() {
    let lines: Vec<_> = BufReader::new(io::stdin()).lines()
    .map(|l| l.unwrap())
    .collect();

    let mut graph = HashMap::<String, Vec<String>>::new();
    for line in lines {
        let ends: Vec<_> = line.split('-').collect();
        let a = ends[0].to_owned();
        let b = ends[1].to_owned();
        graph.entry(a.clone()).or_insert_with(|| vec![]).push(b.clone());
        graph.entry(b).or_insert_with(|| vec![]).push(a);
    }
    // println!("{:?}", graph);

    let mut seen = HashSet::<String>::new();
    let num_routes = visit("start", &graph, &mut seen);

    println!("{}", num_routes);
}

fn visit(start: &str, graph: &HashMap::<String, Vec<String>>, seen: &mut HashSet::<String>) -> u32 {
    let mut num_routes = 0;
    if !('A'..='Z').contains(&start.chars().next().unwrap()) {
        seen.insert(start.to_owned());
    }
    let dests = graph.get(start).unwrap();
    for dest in dests {
        if dest == "end" {
            num_routes += 1;
        } else if !seen.contains(dest) {
            num_routes += visit(dest, graph, seen);
        }
    }
    seen.remove(start);

    num_routes
}
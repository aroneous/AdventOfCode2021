use std::io::{self, prelude::*, BufReader};
use std::collections::VecDeque;

fn main() {
    let lines: Vec<_> = BufReader::new(io::stdin()).lines()
    .map(|l| l.unwrap())
    .collect();

    let width = lines[0].len();

    let mut grid = vec![];
    for line in lines {
        line.chars().map(|c| c.to_string())
        .map(|n| n.parse::<u32>().unwrap()).for_each(|n| grid.push(n));
    }

    let mut cost: Vec<Option<u32>> = vec![None; grid.len()];
    let mut q = VecDeque::<usize>::new();
    q.push_back(grid.len() - 1);
    cost[grid.len() - 1] = Some(0);

    while !q.is_empty() {
        let idx = q.pop_front().unwrap();
        let enter_cost = grid[idx];
        let cell_cost = cost[idx].unwrap();
        for n in neighbors(&grid, width, idx) {
            let better = match cost[n] {
                None => true,
                Some(c) => cell_cost + enter_cost < c
            };
            if better {
                cost[n] = Some(cell_cost + enter_cost);
                if !q.contains(&n) {
                    q.push_back(n);
                }
            }
        }
    }

    println!("{}", cost[0].unwrap());
}

fn neighbors(grid: &Vec<u32>, width: usize, idx: usize) -> Vec<usize> {
    let mut ns = vec![];

    if idx % width != 0 {
        ns.push(idx - 1);
        // if idx >= width + 1 {
        //     ns.push(idx - (width + 1));
        // }
        // if idx + width - 1 < grid.len() {
        //     ns.push(idx + (width - 1));
        // }
    }
    if (idx + 1) % width != 0 && idx + 1 < grid.len() {
        ns.push(idx + 1);
        // if idx >= width - 1 {
        //     ns.push(idx - (width - 1));
        // }
        // if idx + width + 1 < grid.len() {
        //     ns.push(idx + (width + 1));
        // }
    }
    if idx >= width {
        ns.push(idx - width);
    }
    if idx + width < grid.len() {
        ns.push(idx + width);
    }
    return ns;
}
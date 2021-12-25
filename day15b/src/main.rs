use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;

fn main() {
    let lines: Vec<_> = BufReader::new(io::stdin()).lines()
    .map(|l| l.unwrap())
    .collect();

    let width = lines[0].len() * 5;

    let mut grid = vec![];
    for line in lines {
        let row: Vec<_> = line.chars().map(|c| c.to_string())
        .map(|n| n.parse::<u32>().unwrap()).collect();
        for add in 0..5 {
            row.iter().for_each(|n| grid.push(wrap(*n, add)));
        }
    }
    let seglen = grid.len();
    for add in 1..5 {
        for idx in 0..seglen {
            grid.push(wrap(grid[idx], add));
        }
    }

    let mut cost: Vec<Option<u32>> = vec![None; grid.len()];
    let mut to_visit = HashSet::<usize>::new();
    let mut visited = HashSet::<usize>::new();
    cost[grid.len() - 1] = Some(0);
    to_visit.insert(grid.len() - 1);

    while !visited.contains(&0) {
        let idx = least(&cost, &to_visit).unwrap();
        to_visit.remove(&idx);
        visited.insert(idx);
        let enter_cost = grid[idx];
        let cell_cost = cost[idx].unwrap();
        for n in neighbors(&grid, width, idx) {
            let better = match cost[n] {
                None => true,
                Some(c) => cell_cost + enter_cost < c
            };
            if better {
                cost[n] = Some(cell_cost + enter_cost);
                to_visit.insert(n);
            }
        }
    }

    println!("{}", cost[0].unwrap());
}

fn least(cost: &Vec<Option<u32>>, to_visit: &HashSet<usize>) -> Option<usize> {
    to_visit.iter().min_by_key(|idx| cost[**idx].unwrap()).map(|idx| *idx)
}

fn wrap(val: u32, add: u32) -> u32 {
    ((val - 1 + add) % 9) + 1
}

fn neighbors(grid: &Vec<u32>, width: usize, idx: usize) -> Vec<usize> {
    let mut ns = vec![];

    if idx % width != 0 {
        ns.push(idx - 1);
    }
    if (idx + 1) % width != 0 && idx + 1 < grid.len() {
        ns.push(idx + 1);
    }
    if idx >= width {
        ns.push(idx - width);
    }
    if idx + width < grid.len() {
        ns.push(idx + width);
    }
    return ns;
}
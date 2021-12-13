use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;

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

    let mut sum = 0;

    let mut visited = vec![false; grid.len()];
    for level in 0..=9 {
        let mut count = 0;
        for idx in 0..grid.len() {
            if !visited[idx] && grid[idx] == level {
                count += 1;
                walk_area(&grid, width, idx, &mut visited);
            }
        }
        sum += count * (level + 1);
    }

    println!("{}", sum);
}

fn walk_area(grid: &Vec<u32>, width: usize, idx: usize, visited: &mut Vec<bool>) {
    let mut q = HashSet::<usize>::new();
    q.insert(idx);
    while !q.is_empty() {
        let i = *q.iter().next().unwrap();
        q.remove(&i);
        visited[i] = true;
        let val_i = grid[i];
        neighbors(grid, width, i).iter()
        .filter(|j| !visited[**j])
        .filter(|j| grid[**j] >= val_i)
        .for_each(|j| { q.insert(*j); });
    }
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
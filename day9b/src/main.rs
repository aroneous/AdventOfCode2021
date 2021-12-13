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

    let mut visited = vec![false; grid.len()];
    let mut sizes = vec![];
    for level in 0..=9 {
        for idx in 0..grid.len() {
            if !visited[idx] && grid[idx] == level {
                let basin_size = walk_area(&grid, width, idx, &mut visited);
                // println!("basin size ({}, {}) {}", idx / width, idx % width, basin_size);
                sizes.push(basin_size);
            }
        }
    }

    let mut prod = 1;
    sizes.sort();
    sizes.reverse();
    sizes[0..=2].iter().for_each(|size| prod *= size);

    println!("{}", prod);
}

fn walk_area(grid: &Vec<u32>, width: usize, idx: usize, visited: &mut Vec<bool>) -> usize {
    let mut q = HashSet::<usize>::new();
    let mut basin_size = 0;
    q.insert(idx);
    while !q.is_empty() {
        let i = *q.iter().next().unwrap();
        let removed = q.remove(&i);
        if !removed {
            panic!("Remove failed for {}", i);
        }
        visited[i] = true;
        let val_i = grid[i];
        if val_i < 9 {
            // println!("inc for ({}, {})", i / width, i % width);
            basin_size += 1;
        }
        neighbors(grid, width, i).iter()
        .filter(|j| !visited[**j])
        .filter(|j| grid[**j] >= val_i)
        .for_each(|j| { q.insert(*j); });
    }

    basin_size
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
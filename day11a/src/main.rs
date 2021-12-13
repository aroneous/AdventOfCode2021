use std::io::{self, prelude::*, BufReader};

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

    let mut num_flashes = 0;
    for _ in 0..100 {
        let mut visited = vec![false; grid.len()];
        // First, increment all
        for ele in &mut grid {
            *ele += 1;
        }
        // Propagate flashes
        loop {
            let mut done = true;
            for idx in 0..grid.len() {
                if grid[idx] > 9 && !visited[idx] {
                    done = false;
                    num_flashes += 1;
                    visited[idx] = true;
                    for j in neighbors(&grid, width, idx) {
                        grid[j] += 1;
                    }
                }
            }

            if done {
                break;
            }
        }
        // Reset flashed to 0
        for ele in &mut grid {
            if *ele > 9 {
                *ele = 0;
            }
        }
    }

    println!("{}", num_flashes);
}

fn neighbors(grid: &Vec<u32>, width: usize, idx: usize) -> Vec<usize> {
    let mut ns = vec![];

    if idx % width != 0 {
        ns.push(idx - 1);
        if idx >= width + 1 {
            ns.push(idx - (width + 1));
        }
        if idx + width - 1 < grid.len() {
            ns.push(idx + (width - 1));
        }
    }
    if (idx + 1) % width != 0 && idx + 1 < grid.len() {
        ns.push(idx + 1);
        if idx >= width - 1 {
            ns.push(idx - (width - 1));
        }
        if idx + width + 1 < grid.len() {
            ns.push(idx + (width + 1));
        }
    }
    if idx >= width {
        ns.push(idx - width);
    }
    if idx + width < grid.len() {
        ns.push(idx + width);
    }
    return ns;
}
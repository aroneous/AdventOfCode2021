use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;
use regex::Regex;

fn main() {
    let re = Regex::new(r"fold along (x|y)=(\d+)").unwrap();

    let lines: Vec<_> = BufReader::new(io::stdin()).lines()
    .map(|l| l.unwrap())
    .collect();

    // let mut grid = vec![];
    let mut grid = HashSet::<(i32, i32)>::new();
    let mut idx = 0;
    loop {
        let line = &lines[idx];
        if line.len() == 0 {
            break;
        }
        idx += 1;
        let coords: Vec<_> = line.split(',').map(|c| c.parse::<i32>().unwrap()).collect();
        let x = coords[0];
        let y = coords[1];
        // grid.push((x, y));
        grid.insert((x, y));
    }
    // println!("{:?}", grid);

    // viz_grid(&grid);

    idx += 1;
    loop {
        if idx >= lines.len() {
            break;
        }
        let line = &lines[idx];
        let cap = re.captures(line).unwrap();
        let dir = &cap[1];
        let amt = cap[2].parse::<i32>().unwrap();
        idx += 1;

        if dir == "y" {
            // println!("Fold along y == {}", amt);
            grid = translate_grid_y(&grid, amt);
            // println!();
            // viz_grid(&folded);
        } else {
            // println!("Fold along x == {}", amt);
            grid = translate_grid_x(&grid, amt);
            // println!();
            // viz_grid(&folded);
        }
    }
    viz_grid(&grid);
}

fn translate_grid_y(grid: &HashSet<(i32, i32)>, cut: i32) -> HashSet<(i32, i32)> {
    let mut folded = HashSet::<(i32, i32)>::new();
    for (x, y) in grid {
        if *y >= cut {
            let yprime = cut - (y - cut);
            // println!("Folding {} to {}", y, yprime);
            folded.insert((*x, yprime));
        } else {
            folded.insert((*x, *y));
        }
    }

    folded
}

fn translate_grid_x(grid: &HashSet<(i32, i32)>, cut: i32) -> HashSet<(i32, i32)> {
    let mut folded = HashSet::<(i32, i32)>::new();
    for (x, y) in grid {
        if *x >= cut {
            let xprime = cut - (x - cut);
            folded.insert((xprime, *y));
        } else {
            folded.insert((*x, *y));
        }
    }

    folded
}

fn viz_grid(grid: &HashSet<(i32, i32)>) {
    let mut minx = 99999;
    let mut maxx = -99999;
    let mut miny = 99999;
    let mut maxy = -99999;
    for (x, y) in grid {
        if *x < minx {
            minx = *x;
        }
        if *x > maxx {
            maxx = *x;
        }
        if *y < miny {
            miny = *y;
        }
        if *y > maxy {
            maxy = *y;
        }
    }
    for y in miny..=maxy {
        for x in minx..=maxx {
            if grid.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
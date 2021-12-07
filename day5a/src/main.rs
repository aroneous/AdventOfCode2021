use regex::Regex;
use std::io::{self, prelude::*, BufReader};
use std::cmp::{min, max};
use std::collections::HashMap;

fn main() {
    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();


    let ilines: Vec<_> = BufReader::new(io::stdin()).lines()
    .map(|l| l.unwrap())
    .collect();

    let mut lines = Vec::<Line>::new();
    for iline in ilines {
        if re.is_match(&iline) {
            let c = re.captures(&iline).unwrap();
            let line = make_line(
            c[1].parse::<u32>().unwrap(),
            c[2].parse::<u32>().unwrap(),
            c[3].parse::<u32>().unwrap(),
            c[4].parse::<u32>().unwrap(),
            );
            lines.push(line);
        }
    }

    let orth: Vec<_> = lines.iter()
    .filter(|l| l.x1 == l.x2 || l.y1 == l.y2)
    .collect();

    // println!("{:?}", orth);

    let mut counts = HashMap::<Point, usize>::new();

    for line in orth {
        if line.x1 == line.x2 {
            let x = line.x1;
            let start = min(line.y1, line.y2);
            let end = max(line.y1, line.y2);
            for y in start..=end {
                let p = make_point(x, y);
                let curr = *counts.get(&p).unwrap_or(&0);
                counts.insert(p, curr + 1);
            }
        } else {
            let y = line.y1;
            let start = min(line.x1, line.x2);
            let end = max(line.x1, line.x2);
            for x in start..=end {
                let p = make_point(x, y);
                let curr = *counts.get(&p).unwrap_or(&0);
                counts.insert(p, curr + 1);
            }
        }
    }

    let ans = counts.values()
    .filter(|c| **c > 1)
    .count();

    println!("{}", ans);
}

fn make_line(x1:u32, y1:u32, x2:u32, y2:u32) -> Line {
    Line {
        x1,
        y1,
        x2,
        y2,
    }
}

fn make_point(x:u32, y:u32) -> Point {
    Point{x, y}
}

#[derive(Debug)]
struct Line {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

#[derive(Debug)]
#[derive(std::cmp::Eq)]
struct Point {
    x: u32,
    y: u32,
}

impl std::hash::Hash for Point {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        state.write_u32(self.x);
        state.write_u32(self.y);
        state.finish();
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}
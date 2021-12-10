use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;

fn main() {
    let lines: Vec<_> = BufReader::new(io::stdin()).lines()
    .map(|l| l.unwrap())
    .collect();

    let mut total = 0;
    for line in lines {
        let parts: Vec<_> = line.split("|").collect();
        let samples: Vec<_> = parts[0].trim().split_whitespace().collect();
        // let bloop = samples.clone();
        let outputs: Vec<_> = parts[1].trim().split_whitespace().collect();

        let mut ss: Vec<HashSet<char>> = vec![];
        for sample in samples {
            let mut s = HashSet::new();
            sample.chars().for_each(|c| { s.insert(c); });
            ss.push(s);
        }

        let mut os: Vec<HashSet<char>> = vec![];
        for sample in outputs {
            let mut s = HashSet::new();
            sample.chars().for_each(|c| { s.insert(c); });
            os.push(s);
        }

        let one = ss.iter().filter(|s| s.len() == 2).next().unwrap();
        let four = ss.iter().filter(|s| s.len() == 4).next().unwrap();
        let seven = ss.iter().filter(|s| s.len() == 3).next().unwrap();
        let eight = ss.iter().filter(|s| s.len() == 7).next().unwrap();

        // let a = *seven.difference(one).next().unwrap();
        let bd:HashSet<_> = four.difference(one).map(|c| *c).collect();
        
        let five = ss.iter().filter(|s| s.len() == 5)
        .filter(|s| bd.intersection(s).collect::<Vec<_>>().len() == 2).next().unwrap();

        let c = *one.difference(five).next().unwrap();

        let zero = ss.iter().filter(|s| s.len() == 6)
        .filter(|s| bd.difference(s).collect::<Vec<_>>().len() == 1).next().unwrap();

        let six = ss.iter().filter(|s| s.len() == 6)
        .filter(|s| *s != zero)
        .filter(|s| !s.contains(&c)).next().unwrap();

        let nine = ss.iter().filter(|s| s.len() == 6)
        .filter(|s| *s != zero)
        .filter(|s| *s != six).next().unwrap();

        let f = one.iter().filter(|s| **s != c).next().unwrap();

        let two = ss.iter().filter(|s| s.len() == 5)
        .filter(|s| !s.contains(&f)).next().unwrap();

        let three = ss.iter().filter(|s| s.len() == 5)
        .filter(|s| *s != two)
        .filter(|s| *s != five).next().unwrap();

        // println!("\nSamples: {:?}", bloop);
        // println!("zero: {:?}", zero);
        // println!("one: {:?}", one);
        // println!("two: {:?}", two);
        // println!("three: {:?}", three);
        // println!("four: {:?}", four);
        // println!("five: {:?}", five);
        // println!("six: {:?}", six);
        // println!("seven: {:?}", seven);
        // println!("eight: {:?}", eight);
        // println!("nine: {:?}", nine);

        let mut ans = 0;
        for o in os {
            let d = if o == *zero {
                0
            } else if o == *one {
                1
            } else if o == *two {
                2
            } else if o == *three {
                3
            } else if o == *four {
                4
            } else if o == *five {
                5
            } else if o == *six {
                6
            } else if o == *seven {
                7
            } else if o == *eight {
                8
            } else if o == *nine {
                9
            } else {
                panic!("WTF");
            };
            ans = ans * 10 + d;
        }
        // println!("ans {}", ans);
        total += ans;
    }

    println!("{}", total);
}
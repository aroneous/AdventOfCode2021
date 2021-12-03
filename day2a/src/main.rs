use std::io;
use regex::Regex;

fn main() {
    let re = Regex::new(r"(up|down|forward) (\d+)").unwrap();

    let mut buffer = String::new();

    let mut x = 0;
    let mut y = 0;
    while io::stdin().read_line(&mut buffer).is_ok() {
        if !re.is_match(&buffer) {
            break;
        }
        let cap = re.captures(&buffer).unwrap();
        let amt = cap[2].parse::<i32>().unwrap();
        match &cap[1] {
            "up" => {
                y -= amt;
            }

            "down" => {
                y += amt;
            }

            "forward" => {
                x += amt;
            }

            &_ => {
                panic!("WTF command is this?");
            }
        }
        buffer.clear();
    }
    println!("{}", x * y);
}
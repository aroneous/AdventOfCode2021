use std::io;

fn main() {
    let mut count = 0;
    let mut last = -1;
    let mut s0 = -1;
    let mut s1 = -1;
    let mut s2:i32;
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_ok() {
        match buffer.trim().parse::<i32>() {
            Ok(num) => {
                s2 = s1;
                s1 = s0;
                s0 = num;
                if s2 >= 0 {
                    let this = s2 + s1 + s0;
                    if last >= 0 {
                        if this > last {
                            count += 1;
                        }
                    }
                    last = this;
                }
            }
            Err(_) => {
                break;
            }
        }
        // let num = buffer.trim().parse::<i32>().unwrap();
        // if last >= 0 {
        //     if num > last {
        //         count += 1;
        //     }
        // }
        // last = num;
        buffer.clear();
    }
    println!("{}", count);
}

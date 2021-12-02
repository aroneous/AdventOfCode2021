use std::io;

fn main() {
    let mut count = 0;
    let mut last = -1;
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_ok() {
        match buffer.trim().parse::<i32>() {
            Ok(num) => {
                if last >= 0 {
                    if num > last {
                        count += 1;
                    }
                }
                last = num;
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

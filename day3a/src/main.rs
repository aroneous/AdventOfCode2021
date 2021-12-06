use std::io;

fn main() {
    const NUM_BITS:usize = 12;
    let mut buffer = String::new();

    let mut num_recs = 0;
    let mut counts: [i32; NUM_BITS] = [0; NUM_BITS];

    while io::stdin().read_line(&mut buffer).is_ok() {
        if buffer.len() < NUM_BITS {
            break;
        }
        num_recs += 1;
        let chars: Vec<_> = buffer.chars().collect();
        for idx in 0..NUM_BITS {
            match chars[idx] {
                '1' => {
                    counts[idx] += 1;
                }

                _ => {
                }
            }
        }
        buffer.clear();
    }
    let mut gamma = 0;
    let mut mask = 0;
    for idx in 0..NUM_BITS {
        mask = (mask << 1) | 1;
        gamma = gamma << 1;
        if counts[idx] > (num_recs / 2) {
            gamma = gamma | 1;
        }
    }
    let epsilon = !gamma & mask;
    println!("{}", gamma * epsilon);
}
// use std::io;
use std::io::{self, prelude::*, BufReader};

fn main() {
    let nums: Vec<_> = BufReader::new(io::stdin()).lines()
    .map(|l| l.unwrap())
    .filter_map(|l| u32::from_str_radix(&l, 2).ok())
//    .map(|l| l.chars().rev().collect::<Vec<_>>())
    .collect();

    let mut bits: u32 = nums.iter().fold(0, |a, b| a|b);
    let mut num_bits = 0;
    while bits > 0 {
        bits = bits >> 1;
        num_bits += 1;
    }

    // println!("Num bits: {}", num_bits);
    
    // for num in &nums {
    //     // println!("{:012b}", num);
    //     println!("{:0width$b}", num, width = num_bits);
    // }

    // println!("nums: {:?}", nums);

    let mut gen_rate = nums.clone();
    // println!("original gen_rate: {:?}", gen_rate);
    for bit_num in (0..num_bits).rev() {
        let count = gen_rate.iter()
        .fold(0, |acc, num| if num & (1 << bit_num) != 0 { acc + 1 } else { acc });

        // let mut counts = vec![0; num_bits];
        // for bit_num2 in 0..num_bits {
        //     counts[bit_num2] = gen_rate.iter()
        //     .fold(0, |acc, num| if num & (1 << bit_num2) != 0 { acc + 1 } else { acc });
        // }
        // println!("counts: {:?}", counts);
    
        let half = gen_rate.len() as f32 / 2.0;
        // let more_ones: Vec<_> = counts.iter().map(|c| c >= &half).collect();
        let more_ones = count as f32 >= half;
    
        // println!("count: {} half: {} more_ones: {:?}", count, half, more_ones);
    
        gen_rate = gen_rate.iter()
        .filter(|n| {
            let zero_bit = *n & ((1 << bit_num) as u32) == 0;
            // if more_ones[bit_num] { !zero_bit } else { zero_bit }
            if more_ones { !zero_bit } else { zero_bit }
        })
        .map(|n| *n)
        .collect();

        // println!("gen_rate on bit_num {} : {:?}", bit_num, gen_rate);

        if gen_rate.len() == 1 {
            break;
        }
    }

    assert_eq!(gen_rate.len(), 1);

    let gr = gen_rate[0];
    // println!("gen_rate: {}", gr);

    // !!!

    let mut gen_rate = nums.clone();
    // println!("original gen_rate: {:?}", gen_rate);
    for bit_num in (0..num_bits).rev() {
        let count = gen_rate.iter()
        .fold(0, |acc, num| if num & (1 << bit_num) != 0 { acc + 1 } else { acc });

        // let mut counts = vec![0; num_bits];
        // for bit_num2 in 0..num_bits {
        //     counts[bit_num2] = gen_rate.iter()
        //     .fold(0, |acc, num| if num & (1 << bit_num2) != 0 { acc + 1 } else { acc });
        // }
        // println!("counts: {:?}", counts);
    
        let half = gen_rate.len() as f32 / 2.0;
        // let more_ones: Vec<_> = counts.iter().map(|c| c >= &half).collect();
        let more_ones = count as f32 >= half;
    
        // println!("count: {} half: {} more_ones: {:?}", count, half, more_ones);
    
        gen_rate = gen_rate.iter()
        .filter(|n| {
            let zero_bit = *n & ((1 << bit_num) as u32) == 0;
            // if more_ones[bit_num] { !zero_bit } else { zero_bit }
            if !more_ones { !zero_bit } else { zero_bit }
        })
        .map(|n| *n)
        .collect();

        // println!("gen_rate on bit_num {} : {:?}", bit_num, gen_rate);

        if gen_rate.len() == 1 {
            break;
        }
    }

    assert_eq!(gen_rate.len(), 1);

    let csr = gen_rate[0];

    // println!("csr: {}", csr);

    println!("{}", gr * csr);
}
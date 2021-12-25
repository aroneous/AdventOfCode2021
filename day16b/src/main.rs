use std::io::{self, prelude::*, BufReader};
use std::mem::size_of;

fn main() {
    let lines: Vec<_> = BufReader::new(io::stdin()).lines()
    .map(|l| l.unwrap())
    .collect();

    for line in lines {
        let mut pos = 0;
        assert!(line.len() % 2 == 0);
        let mut p = vec![];
        for byte in 0..(line.len()/2) {
            let idx = byte * 2;
            p.push(u8::from_str_radix(&line[idx..(idx+2)], 16).unwrap());
        }
        let value = handle_packet(&p, &mut pos);
        println!("{} == {}", format_hex(&p), value);
    }

    // println!("{}", cost[0].unwrap());
}

fn handle_packet(p:&Vec<u8>, pos:&mut usize) -> u128 {
    // let mut vers_sum = get_bits(&p, *pos, 3);
    // println!("Version: {:b}", vers);
    let t = get_bits(&p, *pos + 3, 3);
    // println!("Type: {:b}", t);
    *pos += 6;
    match t {
        4 => handle_literal_value(p, pos),
        op => handle_operator_packet(op, p, pos)
    }
}

fn handle_literal_value(p:&Vec<u8>, pos:&mut usize) -> u128 {
    let mut value:u128 = 0;
    loop {
        let nibble = get_bits(p, *pos, 5);
        *pos += 5;
        value = (value << 4) + (nibble & 0xf) as u128;
        if nibble & (1 << 4) == 0 {
            break;
        }
    }
    // println!("Literal value {:x}", value);
    value
}

fn handle_operator_packet(op: u32, p:&Vec<u8>, pos:&mut usize) -> u128 {
    let length_type = get_bits(p, *pos, 1);
    *pos += 1;
    let vals = match length_type {
        0 => handle_bit_length_sub_packets(p, pos),
        1 => handle_num_sub_packets(p, pos),
        _ => panic!("Unknown length type {}", length_type)
    };
    match op {
        0 => vals.iter().sum::<u128>(),
        1 => vals.iter().product::<u128>(),
        2 => *vals.iter().min().unwrap(),
        3 => *vals.iter().max().unwrap(),
        5 => if vals[0] > vals[1] { 1 } else { 0 },
        6 => if vals[0] < vals[1] { 1 } else { 0 },
        7 => if vals[0] == vals[1] { 1 } else { 0 },
        _ => panic!("Unknown operator {}", op)
    }
}

fn handle_bit_length_sub_packets(p:&Vec<u8>, pos:&mut usize) -> Vec<u128> {
    let len = get_bits(p, *pos, 15) as usize;
    *pos += 15;
    let end = *pos + len;
    let mut vals = vec![];
    while *pos < end {
        vals.push(handle_packet(p, pos));
    }
    vals
}

fn handle_num_sub_packets(p:&Vec<u8>, pos:&mut usize) -> Vec<u128> {
    let num_packets = get_bits(p, *pos, 11) as usize;
    *pos += 11;
    let mut vals = vec![];
    for _ in 0..num_packets {
        vals.push(handle_packet(p, pos));
    }
    vals
}

fn get_bits(p:&Vec<u8>, start_bit:usize, len:usize) -> u32 {
    assert!(len < size_of::<u32>() * 8);
    let mut ret:u32 = 0;
    for bit in start_bit..start_bit + len {
        let byte = bit / 8;
        let bit = bit % 8;
        let set = p[byte] & (1 << (7 - bit)) != 0;
        ret = ret << 1;
        if set {
            ret = ret | 1;
        }
    }
    ret
}

// fn print_hex(arr:&Vec<u8>) {
//     for byte in arr {
//         print!("{:02X}", byte);
//     }
//     println!();
// }

fn format_hex(arr:&Vec<u8>) -> String {
    let mut s = String::new();
    for byte in arr {
        s.push_str(&format!("{:02X}", byte));
    }
    s
}
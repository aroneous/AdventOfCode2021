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
        let vers_sum = handle_packet(&p, &mut pos);
        println!("{} == {}", format_hex(&p), vers_sum);
    }

    // println!("{}", cost[0].unwrap());
}

fn handle_packet(p:&Vec<u8>, pos:&mut usize) -> u32 {
    let mut vers_sum = get_bits(&p, *pos, 3);
    // println!("Version: {:b}", vers);
    let t = get_bits(&p, *pos + 3, 3);
    // println!("Type: {:b}", t);
    *pos += 6;
    match t {
        4 => handle_literal_value(p, pos),
        _ => vers_sum += handle_operator_packet(p, pos)
    };
    vers_sum
}

fn handle_literal_value(p:&Vec<u8>, pos:&mut usize) {
    loop {
        let nibble = get_bits(p, *pos, 5);
        *pos += 5;
        if nibble & (1 << 4) == 0 {
            break;
        }
    }
}

fn handle_operator_packet(p:&Vec<u8>, pos:&mut usize) -> u32 {
    let length_type = get_bits(p, *pos, 1);
    *pos += 1;
    match length_type {
        0 => handle_bit_length_sub_packets(p, pos),
        1 => handle_num_sub_packets(p, pos),
        _ => panic!("Unknown length type {}", length_type)
    }
}

fn handle_bit_length_sub_packets(p:&Vec<u8>, pos:&mut usize) -> u32 {
    let len = get_bits(p, *pos, 15) as usize;
    *pos += 15;
    let end = *pos + len;
    let mut vers_sum = 0;
    while *pos < end {
        vers_sum += handle_packet(p, pos);
    }
    vers_sum
}

fn handle_num_sub_packets(p:&Vec<u8>, pos:&mut usize) -> u32 {
    let num_packets = get_bits(p, *pos, 11) as usize;
    *pos += 11;
    let mut vers_sum = 0;
    for _ in 0..num_packets {
        vers_sum += handle_packet(p, pos);
    }
    vers_sum
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
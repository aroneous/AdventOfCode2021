use std::io::{self, prelude::*, BufReader};

const BOARD_SIZE:usize = 5;

fn main() {
    let lines: Vec<_> = BufReader::new(io::stdin()).lines()
    .map(|l| l.unwrap())
    .collect();

    let nums: Vec<_> = lines[0].trim().split(",").map(|n| n.parse::<i32>().unwrap())
    .collect();

    let mut boards = Vec::<Vec<i32>>::new();
    let mut idx = 2;
    while idx + BOARD_SIZE <= lines.len() {
        let mut board = Vec::<i32>::new();
        for row in &lines[idx..idx + BOARD_SIZE] {
            row.trim().split_whitespace().map(|n| n.parse::<i32>().unwrap())
            .for_each(|n| board.push(n));
        }
        boards.push(board);
        idx += BOARD_SIZE + 1;
    }

    // println!("nums: {:?}", nums);
    // println!("boards: {:?}", boards);

    let mut marked = vec!(vec!(false; BOARD_SIZE * BOARD_SIZE); boards.len());

    let mut won = false;
    for num in nums {
        for idx in 0..boards.len() {
            let board = &boards[idx];
            let mark = marked.get_mut(idx).unwrap();

            board.iter().enumerate().filter(|(_, n)| **n == num)
            .for_each(|(i, _)| mark[i] = true);
        }
        
        let mut i = 0;
        for mark in &marked {
            if is_win(mark) {
                println!("{}", score(&boards[i], mark, num));
                // println!("Score: {} Win on {} for board {:?}",
                //     score(&boards[i], mark, num), num, boards[i]);
                won = true;
                break;
            }
            i += 1;
        }
        if won { break }
    }
}

fn is_win(marked: &Vec<bool>) -> bool {
    for row in 0..BOARD_SIZE {
        let mut ok = true;
        for col in 0..BOARD_SIZE {
            if !marked[(row * BOARD_SIZE) + col] {
                ok = false;
                break;
            }
        }
        if ok { return true; }
    }

    for col in 0..BOARD_SIZE {
        let mut ok = true;
        for row in 0..BOARD_SIZE {
            if !marked[(row * BOARD_SIZE) + col] {
                ok = false;
                break;
            }
        }
        if ok { return true; }
    }

    false
}

fn score(board: &Vec<i32>, marked: &Vec<bool>, num: i32) -> i32 {
    let unmarked = board.iter().enumerate().filter(|(i, _)| !marked[*i])
    .map(|(_, num)| num).sum::<i32>();
    unmarked * num
}
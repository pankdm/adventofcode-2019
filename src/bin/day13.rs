use std::collections::{HashMap, VecDeque};
use std::io::{self, Write};

use std::io::Read;


extern crate adventofcode;

use adventofcode::*;

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut vm = Vm::from_string(&lines[0]);
    let mut input = VecDeque::new();
    input.push_back(0);

    let res = process_ops(&mut vm, &mut input);

    let mut total = 0;
    let mut index = 0;
    while index < res.len() {
        if res[index + 2] == 2 {
            total += 1;
        }
        index += 3;
    }

    total
}

fn handle(seq: Vec<i64>) -> i64{
    assert!(seq.len() % 3 == 0);
    let mut index = 0;

    let mut score = 0;
    let mut grid = HashMap::new();

    let mut paddle = 0;
    let mut px = 0;
    let mut py = 0;

    let mut ball = 0;
    let mut bx = 0;
    let mut by = 0;

    let mut mx = 0;
    let mut my = 0;

    let mut tiles = 0;

    while index < seq.len() {
        let x = seq[index];
        let y = seq[index + 1];
        let tile = seq[index + 2];
        let mut ch = ' ';
        if x == -1 && y == 0 {
            score = tile;
            index += 3;
            continue;
        } 
        mx = mx.max(x);
        my = my.max(y);        

        if tile == 0 {
            ch = ' ';
        } else if tile == 1 {
            ch = '#';
            tiles += 1;
        } else if tile == 2 {
            ch = 'X';
        } else if tile == 3 {
            ch = '=';
            paddle += 1;
            px = x;
            py = y;
        } else if tile == 4 {
            ch = 'o';
            bx = x;
            by = y;
            ball += 1;
        } else {
            assert!(false);
        }
        grid.insert((x, y), ch);
        index += 3;
    }


    for y in 0..my + 1 {
        for x in 0..mx + 1 {
            let mut ch = '?';
            let key = (x, y);
            match grid.get(&key) {
                Some(&new_ch) => {
                    ch = new_ch;
                }
                _ => {}
            }
            print!("{}", ch);
        }
        println!("");
    }
    

    println!("score = {}", score);
    println!("paddle = {} at ({}, {})", paddle, px, py);
    println!("ball = {} at ({}, {})", ball, bx, by);

    // if px == bx {
    //     return 0;
    // }
    // if px < bx {
    //     return 1;
    // } 
    // if px > bx {
    //     return -1;
    // }

    return tiles;
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut vm = Vm::from_string(&lines[0]);

    vm.ops[0] = 2;

    let mut cnt = 0;

    loop {
        cnt += 1;
        println!("\n");
        println!(" >> cnt = {}", cnt);

        // let rg: i64 = task_rng().gen_range(0, 10);
        // input = (rg % 3) - 1;
        // if cnt < ins.len() {
        //     input = ins[cnt];
        // } else {
        //     input = last_input;
        // }

        let in_tmp: Option<i32> = std::io::stdin()
            .bytes() 
            .next()
            .and_then(|result| result.ok())
            .map(|byte| byte as i32);

        let in_char = in_tmp.unwrap() as u8 as char;
        
        let mut value = 0;
        if in_char == 'a' {
            value = -1;
        } else if in_char == 's' {
            value = 0;
        } else if in_char == 'd' {
            value = 1;
        } else {
            value = 0;
            // continue;
        }

        let mut input = VecDeque::new();
        input.push_back(value);

        let res = process_ops(&mut vm, &mut input);
        if vm.halted {
            println!("game over!");
            break;
        }
        let tiles = handle(res);
        println!("processing INPUT {} --> tiles {}", value, tiles);
        if tiles == 0 {
            break;
        }

        // if cnt > 100 {
        //     break;
        // }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_input("day13/in.txt");
        assert_eq!(part1(&lines), 242);
    }

//     #[test]
//     fn test_part2() {
//         let lines = read_input("day11/in.txt");
//         // assert_eq!(part2(&lines), 46643);
//     }
}

fn main() {
    let lines = read_input("day13/in.txt");

    // println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}

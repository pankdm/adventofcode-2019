use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;
use std::collections::VecDeque;
use std::collections::HashMap;
use std::io::{self, Write};

extern crate adventofcode;
use adventofcode::*;

use std::time::{Duration, Instant};


fn get_value(ops: &Vec<i64>, value: i64, mode: i64, base: i64) -> i64 {
    if mode == 0 {
        return ops[value as usize];
    } else if mode == 1 {
        return value;
    } else if mode == 2 {
        return ops[(value + base) as usize];
    }
    assert!(false);
    return -1;
}

fn get_pos(value: i64, mode: i64, base: i64) -> i64 {
    let mut pos = 0;
    if mode == 0 {
        pos = value
    } else if mode == 2 {
        pos = value + base;
    } else {
        assert!(false);
    }
    return pos;
}

#[derive(Clone)]
struct Vm {
    ops: Vec<i64>,
    index: usize,
    base: i64,
}


fn process_ops(vm: &mut Vm, input: &mut VecDeque<i64>) -> Vec<i64> {
    let ref mut ops = vm.ops;
    let mut res = Vec::new();

    // println!("processing ops: {:?}", ops);

    let mut index = vm.index;
    let mut base = vm.base;
    let mut read_input = false;
    while index < ops.len() {
        let mut value = ops[index];
        // println!("execute {}", value);

        let op = value % 100;
        value /= 100;

        let ma = value % 10;
        value /= 10;

        let mb = value % 10;
        value /= 10;

        let mc = value % 10;

        if op == 99 {
            break;
        } else {
            if op == 1 {
                let a = ops[index + 1];
                let b = ops[index + 2];
                let c = ops[index + 3];
                // assert!(mc == 0);
                let pos = get_pos(c, mc, base);
                ops[pos as usize] = get_value(&ops, a, ma, base) + get_value(&ops, b, mb, base);
                index += 4;
            } else if op == 2 {
                let a = ops[index + 1];
                let b = ops[index + 2];
                let c = ops[index + 3];
                let pos = get_pos(c, mc, base);
                ops[pos as usize] = get_value(&ops, a, ma, base) * get_value(&ops, b, mb, base);
                index += 4;
            } else if op == 3 {
                let a = ops[index + 1];
                // if read_input {
                //     vm.index = index;
                //     vm.base = base;
                //     return res;
                // }
                // read_input = true;
                // assert!(ma == 0);
                // ops[a as usize] = get_value(&ops, input, 0);
                let pos = get_pos(a, ma, base);
                assert!(input.len() > 0);
                ops[pos as usize] = input.pop_front().unwrap();
                index += 2;
            } else if op == 4 {
                let a = ops[index + 1];
                let out = get_value(&ops, a, ma, base);
                res.push(out);
                // println!("   >>> {}", out);
                index += 2;
            } else if op == 5 {
                let a = ops[index + 1];
                let b = ops[index + 2];
                if get_value(&ops, a, ma, base) != 0 {
                    index = get_value(&ops, b, mb, base) as usize;
                } else {
                    index += 3;
                }
            } else if op == 6 {
                let a = ops[index + 1];
                let b = ops[index + 2];
                if get_value(&ops, a, ma, base) == 0 {
                    index = get_value(&ops, b, mb, base) as usize;
                } else {
                    index += 3;
                }
            } else if op == 7 {
                let a = ops[index + 1];
                let b = ops[index + 2];
                let c = ops[index + 3];
                let pos = get_pos(c, mc, base);
                if get_value(&ops, a, ma, base) < get_value(&ops, b, mb, base) {
                    ops[pos as usize] = 1;
                } else {
                    ops[pos as usize] = 0;
                }
                index += 4;

            } else if op == 8 {
                let a = ops[index + 1];
                let b = ops[index + 2];
                let c = ops[index + 3];
                // assert!(mc == 0);
                let pos = get_pos(c, mc, base);
                if get_value(&ops, a, ma, base) == get_value(&ops, b, mb, base) {
                    ops[pos as usize] = 1;
                } else {
                    ops[pos as usize] = 0;
                }
                index += 4;
            } else if op == 9 {
                let a = ops[index + 1];
                base += get_value(&ops, a, ma, base);
                index += 2;
            } else {
                println!("Unknown op: {}", op);
                assert!(false);
            }
        }
    }
    return res;
}


fn print_map(res: &Vec<i64>) {
    for code in res {
        print!("{}", *code as u8 as char);
    }
}



pub fn part1(lines: &Vec<String>) -> i64 {
    let mut str_ops = lines[0].split(",").collect::<Vec<&str>>();
    // println!("ops: {:?}", ops);

    let mut ops = Vec::new();
    for str_op in str_ops {
        ops.push(str_op.parse::<i64>().unwrap());
    }

    while ops.len() < 10000 {
        ops.push(0);
    }

    let mut vm = Vm {
        ops, index: 0, base: 0
    };


    let mut input = VecDeque::new();

    let res = process_ops(&mut vm, &mut input);
    print_map(&res);


    let mut grid = Vec::new();
    let mut row = Vec::new();
    for code in &res {
        if *code == 10 {
            if row.len() > 0 {
                grid.push(row.clone());
                row = Vec::new();
            }
        } else {
            row.push(*code as u8 as char);
        }
    }

    let mut ans = 0 as i64;
    for y in 1..(grid.len() - 1) {
        for x in 1..(grid[y].len() - 1) {
            if grid[y][x] == '#' && grid[y + 1][x] == '#' && grid[y - 1][x] == '#' 
                && grid[y][x + 1] == '#' && grid[y][x - 1] == '#' {
                    ans += (x as i64) * (y as i64);
                }
                
        }
    }


    for code in &res {
        if *code == 10 {
            if row.len() > 0 {
                grid.push(row.clone());
                row = Vec::new();
            }
        } else {
            row.push(*code as u8 as char);
        }
    }

    let mut x0 = 0;
    let mut y0 = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '^' {
                x0 = x;
                y0 = y;
            }
        }
    }

    grid[y0][x0] = '#';

    let pos = Pos{x : x0 as i64, y: y0 as i64};
    let path = traverse_path(&grid, pos);

    let mut ans = 0 as i64;

    return ans;
}


type Grid = Vec<Vec<char>>;

fn get_char(grid: &Grid, pos: Pos) -> char {
    let x = pos.x;
    let y = pos.y;
    if y < 0 || y >= grid.len() as i64 || x < 0 || x >= grid[y as usize].len() as i64 {
        return '.';
    }
    return grid[y as usize][x as usize];
}


fn is_good(grid: &Grid, pos: Pos) -> bool {
    return get_char(grid, pos) == '#';
}

#[derive(Clone, Copy)]
pub struct Dir {
    dx: i64,
    dy: i64,
}

#[derive(Clone, Copy)]
pub struct Pos {
    x: i64,
    y: i64,
}

fn next_pos(p: Pos, d: Dir) -> Pos {
    return Pos {
        x: p.x + d.dx,
        y: p.y + d.dy,
    };
}

fn rotate_left(d: Dir) -> Dir {
    return Dir {
        dx: d.dy,
        dy: -d.dx,
    }
}

fn rotate_right(d: Dir) -> Dir {
    return Dir {
        dx: -d.dy,
        dy: d.dx,
    }
}


pub fn traverse_path(grid: &Grid, xy: Pos) {
    let mut dir = Dir {
        dx: 0, dy : -1
    };

    let mut pos = xy;
    let mut steps = 0;
    let mut res = Vec::new();

    loop {
        let pos_forward = next_pos(pos, dir);
        if is_good(grid, pos_forward) {
            steps += 1;
            pos = pos_forward.clone();
            continue;
        }

        let ld = rotate_left(dir);
        let pos_left = next_pos(pos, ld);
        if is_good(grid, pos_left) {
            res.push(steps.to_string());
            res.push("L".to_string());
            steps = 0;
            dir = ld;
            continue;
        }

        let rd = rotate_right(dir);
        let pos_right = next_pos(pos, rd);
        if is_good(grid, pos_right) {
            res.push(steps.to_string());
            res.push("R".to_string());
            steps = 0;
            dir = rd;
            continue;
        }

        res.push(steps.to_string());
        break;
    }
    // println!("{:?}", &res);
    for r in &res {
        print!("{},", r);
    }
    println!("");

}


pub fn part2(lines: &Vec<String>) -> i64 {
    let mut str_ops = lines[0].split(",").collect::<Vec<&str>>();
    // println!("ops: {:?}", ops);

    let mut ops = Vec::new();
    for str_op in str_ops {
        ops.push(str_op.parse::<i64>().unwrap());
    }

    while ops.len() < 10000 {
        ops.push(0);
    }

    ops[0] = 2;
    let mut vm = Vm {
        ops, index: 0, base: 0
    };

    let mut input = VecDeque::new();
    let lines = r#"
A,B,B,C,A,B,C,A,B,C
L,6,R,12,L,4,L,6
R,6,L,6,R,12
L,6,L,10,L,10,R,6
n
"#.trim();

    for ch in lines.chars() {
        input.push_back(ch as i64);
    }
    input.push_back('\n' as i64);

    let res = process_ops(&mut vm, &mut input);
    print_map(&res);
    println!("");
    println!("{:?}", res[res.len() - 1]);
    -1
}


fn main() {
    let lines = read_input("day17/in.txt");

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));

}
use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{self, Write};
use std::io::{BufRead, BufReader};
use std::process;

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

fn is_beam(x: i64, y: i64, ops: Vec<i64>) -> bool {
    let mut vm = Vm {
        ops: ops.clone(),
        index: 0,
        base: 0,
    };
    let mut input = VecDeque::new();
    input.push_back(x);
    input.push_back(y);
    let res = process_ops(&mut vm, &mut input);
    assert!(res.len() == 1);
    return res[0] == 1;
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

    // print_map(&res);
    let mut ans = 0 as i64;

    let mut grid = Vec::new();
    for y in 0..50 {
        let mut row = Vec::new();
        for x in 0..50 {
            let mut vm = Vm {
                ops: ops.clone(),
                index: 0,
                base: 0,
            };
            let mut input = VecDeque::new();
            input.push_back(x);
            input.push_back(y);
            let res = process_ops(&mut vm, &mut input);
            let mut ch = ' ';
            assert!(res.len() == 1);
            if res[0] == 1 {
                ch = '#';
                ans += 1;
            } else {
                ch = ' ';
            }
            row.push(ch);
            print!("{}", ch);
        }
        grid.push(row);
        println!("");
    }

    return ans;
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
    let mut beam = HashSet::new();

    let WIDTH = 100;

    let mut dpx = HashMap::new();
    let mut dpy = HashMap::new();

    let mut cy = 7;
    let mut xmin = 0;
    loop {
        while !is_beam(xmin, cy, ops.clone()) {
            xmin += 1;
        }
        assert!(is_beam(xmin, cy, ops.clone()));
        let mut xmax = xmin;
        while is_beam(xmax, cy, ops.clone()) {
            beam.insert((xmax, cy));
            xmax += 1;
        }

        for x in xmin..xmax {
            let key = (x, cy);
            dpx.insert(key, x - xmin + 1);
            match dpy.get(&(x, cy - 1)) {
                Some(&value) => {
                    dpy.insert(key, value + 1);
                }
                _ => {
                    dpy.insert(key, 1);
                }
            }
            if dpx[&key] >= WIDTH && dpy[&key] >= WIDTH {
                let xans = x - WIDTH + 1;
                let yans = cy - WIDTH + 1;
                println!("ans at X={}, Y={}", xans, yans);
                return xans * 10000 + yans;
            }
        }
        cy += 1;
        if cy % 100 == 0 {
            println!("at {}", cy);
        }
    }
}

fn is_beam_file(x: i64, y: i64, beam: &HashSet<(i64, i64)>) -> bool {
    return beam.contains(&(x, y));
}

pub fn part2_file(lines: &Vec<String>) -> i64 {
    let mut beam = HashSet::new();
    for y in 0..lines.len() {
        let mut x = 0;
        for ch in lines[y].chars() {
            if ch == '#' || ch == 'O' {
                beam.insert((x as i64, y as i64));
            }
            x += 1;
        }
    }

    let WIDTH = 10;

    let mut dpx = HashMap::new();
    let mut dpy = HashMap::new();

    let mut cy = 7;
    let mut xmin = 0;
    loop {
        while !is_beam_file(xmin, cy, &beam) {
            xmin += 1;
        }
        assert!(is_beam_file(xmin, cy, &beam));
        let mut xmax = xmin;
        while is_beam_file(xmax, cy, &beam) {
            beam.insert((xmax, cy));
            xmax += 1;
        }

        for x in xmin..xmax {
            let key = (x, cy);
            dpx.insert(key, x - xmin + 1);
            match dpy.get(&(x, cy - 1)) {
                Some(&value) => {
                    dpy.insert(key, value + 1);
                }
                _ => {
                    dpy.insert(key, 1);
                }
            }
            if dpx[&key] >= WIDTH && dpy[&key] >= WIDTH {
                println!("at {:?} -> dpx = {}, dpy = {}", &key, dpx[&key], dpy[&key]);
                let xans = x - WIDTH + 1;
                let yans = cy - WIDTH + 1;
                println!("ans at X={}, Y={}", xans, yans);
                return xans * 10000 + yans;
            }
        }
        cy += 1;
        if cy % 100 == 0 {
            println!("at {}", cy);
        }
    }
}

fn main() {
    let lines = read_input("day19/in.txt");

    // println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));

    // let lines = read_input("day19/t0.txt");
    // println!("part2 = {}", part2_file(&lines));
}

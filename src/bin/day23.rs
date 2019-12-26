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

fn process_ops_once(vm: &mut Vm, input: &mut VecDeque<i64>, res: &mut VecDeque<i64>) {
    let ref mut ops = vm.ops;
    // let mut res = Vec::new();

    // println!("processing ops: {:?}", ops);

    let mut index = vm.index;
    let mut base = vm.base;
    // let mut read_input = false;
    // while index < ops.len() {
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
        println!("Terminated!");
        return;
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
            let mut value = -1;
            if (!input.is_empty()) {
                value = input.pop_front().unwrap();
            }
            // if read_input {
            //     vm.index = index;
            //     vm.base = base;
            //     return res;
            // }
            // read_input = true;
            // assert!(ma == 0);
            // ops[a as usize] = get_value(&ops, input, 0);
            let pos = get_pos(a, ma, base);
            // assert!(input.len() > 0);
            ops[pos as usize] = value;
            index += 2;
        } else if op == 4 {
            let a = ops[index + 1];
            let out = get_value(&ops, a, ma, base);
            res.push_back(out);
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

    vm.index = index;
    vm.base = base;
    // }
    // return res;
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

    let mut vms = Vec::new();
    let mut inputs = Vec::new();
    let mut outputs = Vec::new();

    for i in 0..50 {
        let mut input = VecDeque::new();
        input.push_back(i);

        let mut vm = Vm {
            ops: ops.clone(),
            index: 0,
            base: 0,
        };

        let mut output = VecDeque::new();

        vms.push(vm);
        inputs.push(input);
        outputs.push(output);
    }

    loop {
        for i in 0..50 {
            println!("Machine {} at index = {}", i, vms[i].index);
            process_ops_once(&mut vms[i], &mut inputs[i], &mut outputs[i]);

            if outputs[i].len() >= 3 {
                let addr = outputs[i].pop_front().unwrap();
                let x = outputs[i].pop_front().unwrap();
                let y = outputs[i].pop_front().unwrap();

                println!("Machine {} sent X={},Y={} to {}", i, x, y, addr);

                if addr == 255 {
                    return y;
                }

                if 0 <= addr && addr < 50 {
                    inputs[addr as usize].push_back(x);
                    inputs[addr as usize].push_back(y);
                } else {
                    unreachable!();
                }
            }
        }
    }
    unreachable!();
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

    let mut vms = Vec::new();
    let mut inputs = Vec::new();
    let mut outputs = Vec::new();
    let mut last_send = Vec::new();

    for i in 0..50 {
        let mut input = VecDeque::new();
        input.push_back(i);

        let mut vm = Vm {
            ops: ops.clone(),
            index: 0,
            base: 0,
        };

        let mut output = VecDeque::new();

        vms.push(vm);
        inputs.push(input);
        outputs.push(output);
        last_send.push(0);
    }

    let mut natx = 0;
    let mut naty = 0;
    let mut cnt = 0;
    let mut time = 0;
    let mut TIMEOUT = 100 as i64;
    let mut is_idle = false;

    loop {
        for i in 0..50 {
            // println!("Machine {} at index = {}", i, vms[i].index);
            process_ops_once(&mut vms[i], &mut inputs[i], &mut outputs[i]);

            if outputs[i].len() >= 3 {
                let addr = outputs[i].pop_front().unwrap();
                let x = outputs[i].pop_front().unwrap();
                let y = outputs[i].pop_front().unwrap();

                // println!("Machine {} sent X={},Y={} to {}", i, x, y, addr);

                if addr == 255 {
                    natx = x;
                    naty = y;
                    if is_idle {
                        println!("   ({}) NAT got X={},Y={}", cnt, x, y);
                        cnt += 1;
                        inputs[0].push_back(x);
                        inputs[0].push_back(y);
                        is_idle = false;
                        // if cnt == 2 {
                        //     return y;
                        // }
                    }
                    continue;
                }

                if 0 <= addr && addr < 50 {
                    inputs[addr as usize].push_back(x);
                    inputs[addr as usize].push_back(y);

                    last_send[i] = time;
                    is_idle = false;
                } else {
                    unreachable!();
                }
            }
        }
        time += 1;
        if time % 1000 == 0 {
            // println!(" ({}) checking for idle", time);
        }
        // check if idle
        let mut ok = true;
        for i in 0..50 {
            if inputs[i].len() > 0 {
                ok = false;
                break;
            }
            if last_send[i] + TIMEOUT > time {
                ok = false;
                break;
            }
        }
        if ok {
            is_idle = true;
        }
    }
    unreachable!();
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_part1() {
//         let lines = read_input("day19/in.txt");
//         assert_eq!(part1(&lines), 150);
//     }

//     #[test]
//     #[ignore]
//     fn test_part2() {
//         let lines = read_input("day19/in.txt");
//         assert_eq!(part2(&lines), 12201460);
//     }
// }

fn main() {
    let lines = read_input("day23/in.txt");

    // println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));

    // let lines = read_input("day19/t0.txt");
    // println!("part2 = {}", part2_file(&lines));
}

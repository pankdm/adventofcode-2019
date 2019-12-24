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
        ops,
        index: 0,
        base: 0,
    };

    // jump = any of A,B,C is false (hole) + D is ground (true)
    // jump = (!A OR !B OR !C) AND D
    // jump = !(A AND B AND C) AND D

    let mut prog = r#"
NOT T T
AND A T
AND B T
AND C T
NOT T T
AND D T
OR T J
WALK
"#.trim();

    let mut input = VecDeque::new();
    for c in prog.chars() {
        input.push_back(c as u8 as i64);
    }
    input.push_back('\n' as u8 as i64);

    let res = process_ops(&mut vm, &mut input);
    for &v in &res {
        if v <= 255 {
            print!("{}", v as u8 as char);
        }
    }
    let mut code = -1;
    for i in 0..res.len() {
        let v = res[i];
        if v > 255 {
            println!("at {}, v = {}", i, v);
            code = v;
        }
    }
    code
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
    let mut vm = Vm {
        ops,
        index: 0,
        base: 0,
    };

    // jump = any of A,B,C is false (hole) + D is ground (true)
    // jump = (!A OR !B OR !C) AND D
    // jump = !(A AND B AND C) AND D

    // ###.###.#...#.
    //   0123456789
    // 1 A
    // 2 B
    // 3 C
    // 4 D
    // 5 E
    // 6 F
    // 7 G
    // 8 H
    // 9 I
    // jump = (need and can jump at 0)
    //    and (5 || 8)
    //       for 5 = 6 || 9
    // jump = (!(A and B and C) and D) and (H or (E and (F || I)

    let mut prog = r#"
NOT T T
AND A T
AND B T
AND C T
NOT T T
AND D T
OR I J
OR F J
AND E J
OR H J
AND T J
RUN
"#.trim();

    let mut input = VecDeque::new();
    for c in prog.chars() {
        input.push_back(c as u8 as i64);
    }
    input.push_back('\n' as u8 as i64);

    let res = process_ops(&mut vm, &mut input);
    for &v in &res {
        if v <= 255 {
            print!("{}", v as u8 as char);
        }
    }
    let mut code = -1;
    for i in 0..res.len() {
        let v = res[i];
        if v > 255 {
            println!("at {}, v = {}", i, v);
            code = v;
        }
    }
    code
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
    let lines = read_input("day21/in.txt");

    // println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));

    // let lines = read_input("day19/t0.txt");
    // println!("part2 = {}", part2_file(&lines));
}

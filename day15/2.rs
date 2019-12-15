use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;
use std::collections::VecDeque;
use std::collections::HashMap;
use std::io::{self, Write};



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


fn process_ops(vm: &mut Vm, input: i64) -> Vec<i64> {
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
                if read_input {
                    vm.index = index;
                    vm.base = base;
                    return res;
                }
                read_input = true;
                // assert!(ma == 0);
                // ops[a as usize] = get_value(&ops, input, 0);
                let pos = get_pos(a, ma, base);
                ops[pos as usize] = input;
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

fn wrapper(vm: &mut Vm, input: i64) -> i64 {
    let res = process_ops(vm, input);
    assert!(res.len() == 1, "{:?}", res);
    return res[0];
}


fn print_map(dist: &HashMap<(i64, i64), i64>, x00: i64, y00: i64) {
    let mut x_min = 0;
    let mut x_max = 0;
    let mut y_min = 0;
    let mut y_max = 0;

    for (k, v) in dist.iter() {
        let (x, y) = *k;
        x_min = x_min.min(x);
        x_max = x_max.max(x);
        y_min = y_min.min(y);
        y_max = y_max.max(y);
        // println!("{:?} --> {}", k, v);
    }

    for y in y_min..(y_max + 1) {
        for x in x_min..(x_max + 1) {
            if x == x00 && y == y00 {
                print!("O");
                continue;
            }
            if x == 0 && y == 0 {
                print!("D");
                continue;
            }
            if dist.contains_key(&(x, y)) {
                let v = dist[&(x, y)];
                if v == -1 {
                    print!("#");
                } else {
                    print!(" ");
                }
            } else {
                print!(".");
            }
        }
        print!("\n");
        io::stdout().flush().unwrap();
    }
}

fn find_oxygen(ops: Vec<i64>) -> (i64, i64, Vm) {
    let mut vm_start = Vm {
        ops, index: 0, base: 0
    };

    let mut dist = HashMap::new();
    let mut q = VecDeque::new();

    let mut dirs = vec![ (1, -1, 0), (2, 1, 0), (3, 0, -1), (4, 0, 1) ];

    q.push_back((vm_start, (0, 0), 0));
    while !q.is_empty() {
        let (vm, pos, d) = q.pop_front().unwrap();
        let (x0, y0) = pos;
        for (code, dx, dy) in &dirs {
            let mut new_vm = vm.clone();
            let x = x0 + dx;
            let y = y0 + dy;
            if dist.contains_key(&(x, y)) {
                continue;
            }
            let status = wrapper(&mut new_vm, *code);
            if status == 2 {
                print_map(&dist, x, y);
                return (x, y, new_vm);
            } 
            if status == 0 {
                dist.insert((x, y), -1);
                continue;
            }
            dist.insert((x, y), d + 1);
            q.push_back( (new_vm, (x, y), d + 1) );
        }
    }
    assert!(false);
    unreachable!();
}

fn find_time(ops: Vec<i64>) -> i64 {
    let (x00, y00, vm_start) = find_oxygen(ops.clone());

    println!("  ");
    let mut dist = HashMap::new();
    let mut q = VecDeque::new();

    let mut dirs = vec![ (1, -1, 0), (2, 1, 0), (3, 0, -1), (4, 0, 1) ];

    q.push_back((vm_start, (x00, y00), 0));
    while !q.is_empty() {
        let (vm, pos, d) = q.pop_front().unwrap();
        let (x0, y0) = pos;
        for (code, dx, dy) in &dirs {
            let mut new_vm = vm.clone();
            let x = x0 + dx;
            let y = y0 + dy;
            if dist.contains_key(&(x, y)) {
                continue;
            }
            let status = wrapper(&mut new_vm, *code);
            if status == 0 {
                dist.insert((x, y), -1);
                continue;
            }
            dist.insert((x, y), d + 1);
            q.push_back( (new_vm, (x, y), d + 1) );
        }
    }

    let mut best = 0;
    for (k, v) in dist.iter() {
        best = best.max(*v);
    }


    print_map(&dist, x00, y00);


    return best;
}

fn main() {
    // let filename = "t2.txt";
    let filename = "in.txt";

    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line);

    // println!("read: {}", line);

    let mut str_ops = line.split(",").collect::<Vec<&str>>();
    // println!("ops: {:?}", ops);

    let mut ops = Vec::new();
    for str_op in str_ops {
        ops.push(str_op.parse::<i64>().unwrap());
    }

    while ops.len() < 10000 {
        ops.push(0);
    }

    let ans = find_time(ops.clone());

    println!("ans = {}", ans);
}
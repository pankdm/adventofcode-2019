use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;
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


fn process_ops(ops: &mut Vec<i64>, input: i64, _index: &mut usize, _base: &mut i64, halted: &mut bool) -> Vec<i64> {
    let mut index = *_index;
    let mut base = *_base;
    println!("starting from index = {}, base = {}", index, base);

    let mut res = Vec::new();

    let mut read_input = false;

    // println!("processing ops: {:?}", ops);
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
            *halted = true;
            break;
        } else {
            if op == 1 {
                let a = ops[index + 1];
                let b = ops[index + 2];
                let c = ops[index + 3];
                // assert!(mc == 0);
                let pos = get_pos(c, mc, base);
                ops[pos as usize] = get_value(ops, a, ma, base) + get_value(ops, b, mb, base);
                index += 4;
            } else if op == 2 {
                let a = ops[index + 1];
                let b = ops[index + 2];
                let c = ops[index + 3];
                let pos = get_pos(c, mc, base);
                ops[pos as usize] = get_value(ops, a, ma, base) * get_value(ops, b, mb, base);
                index += 4;
            } else if op == 3 {
                if read_input {
                    assert!(res.len() == 2);
                    *_index = index;
                    *_base = base;
                    println!("  >> returning from index {}, base {}", index, base);
                    return res;
                }
                read_input = true;

                let a = ops[index + 1];
                // assert!(ma == 0);
                // ops[a as usize] = get_value(&ops, input, 0);
                let pos = get_pos(a, ma, base);
                ops[pos as usize] = input;
                index += 2;
            } else if op == 4 {
                let a = ops[index + 1];
                let out = get_value(ops, a, ma, base);
                res.push(out);
                // println!("   >>> {}", out);
                index += 2;
            } else if op == 5 {
                let a = ops[index + 1];
                let b = ops[index + 2];
                if get_value(ops, a, ma, base) != 0 {
                    index = get_value(ops, b, mb, base) as usize;
                } else {
                    index += 3;
                }
            } else if op == 6 {
                let a = ops[index + 1];
                let b = ops[index + 2];
                if get_value(&ops, a, ma, base) == 0 {
                    index = get_value(ops, b, mb, base) as usize;
                } else {
                    index += 3;
                }
            } else if op == 7 {
                let a = ops[index + 1];
                let b = ops[index + 2];
                let c = ops[index + 3];
                let pos = get_pos(c, mc, base);
                if get_value(ops, a, ma, base) < get_value(ops, b, mb, base) {
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
                if get_value(ops, a, ma, base) == get_value(ops, b, mb, base) {
                    ops[pos as usize] = 1;
                } else {
                    ops[pos as usize] = 0;
                }
                index += 4;
            } else if op == 9 {
                let a = ops[index + 1];
                base += get_value(ops, a, ma, base);
                index += 2;
            } else {
                println!("Unknown op: {}", op);
                assert!(false);
            }
        }
    }
    return res;
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

    let mut grid = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    grid.entry((x, y)).or_insert(1);
    let mut dx = 0;
    let mut dy = -1;
    let mut halted = false;
    let mut index = 0 as usize;
    let mut base = 0 as i64;
    loop {
        let key = (x, y);
        let mut input = 0;
        match grid.get(&key) {
            Some(& value) => {
                input = value;
            }
            _ => {}
        }
        let res = process_ops(&mut ops, input, &mut index, &mut base, &mut halted);
        if halted {
            break;
        }
        grid.insert(key, res[0]);
        if res[1] == 1 {
            let dytmp = dy;
            let dxtmp = dx;
            dx = dytmp;
            dy = -dxtmp;
        } else {
            let dytmp = dy;
            let dxtmp = dx;
            dx = -dytmp;
            dy = dxtmp;
        }
        x += dx;
        y += dy;
    }
    // println!("tiles = {}", grid.len());
    
    let mut minx = 0;
    let mut maxx = 0;
    let mut miny = 0;
    let mut maxy = 0;

    let grid_copy = grid.clone();

    for (k, v) in grid_copy {
        x = k.0;
        y = k.1;

        minx = minx.min(x);
        maxx = maxx.max(x);

        miny = miny.min(y);
        maxy = maxy.max(y);
    }

    for y in miny..maxy + 1 {
        for x in (minx..maxx + 1).rev() {
            let key = (x, y);
            let color = grid.entry(key).or_default();
            if *color == 0 {
                print!(" ");
            } else {
                print!("#");
            }
        }
        print!("\n");
        io::stdout().flush().unwrap();
    }
}
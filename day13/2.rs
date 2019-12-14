use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;
use std::collections::HashMap;
use std::io::{self, Write};

use std::io::Read;

// use std::rand::{task_rng, Rng};


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


fn process_ops(ops: &mut Vec<i64>,  _index: &mut usize, _base: &mut i64, input: i64, read_input: &mut bool, halted: &mut bool) -> Vec<i64> {
    let mut index = *_index;
    let mut base = *_base;    // let mut ops = ops_tmp.clone();
    let mut res = Vec::new();

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
                if *read_input {
                    return res;
                }
                *read_input = true;
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
        print!("\n");
        io::stdout().flush().unwrap();

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

    ops[0] = 2;


    let mut cnt = 0;

    let mut last_input = 0;
    let mut input = 0;

    let mut ins = Vec::new();
    for i in 0..0 {
        ins.push(-1);
    }


    let mut index = 0;
    let mut base = 0;
    let mut read_input = false;
    let mut halted = false;


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
        
        if in_char == 'a' {
            input = -1;
        } else if in_char == 's' {
            input = 0;
        } else if in_char == 'd' {
            input = 1;
        } else {
            continue;
        }

        let res = process_ops(&mut ops, &mut index, &mut base, input, &mut read_input, &mut halted);
        read_input = false;
        if halted {
            println!("game over!");
            break;
        }
        let tiles = handle(res);
        println!("processing INPUT {} --> tiles {}", input, tiles);
        if tiles == 0 {
            break;
        }

        // if cnt > 100 {
        //     break;
        // }
    }
}
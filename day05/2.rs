use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;


fn get_value(ops: &Vec<i64>, value: i64, mode: i64) -> i64 {
    if mode == 0 {
        return ops[value as usize];
    } else if mode == 1 {
        return value;
    }
    assert!(false);
    return -1;
}


fn process_ops(ops_tmp: Vec<i64>, input: i64) -> Vec<i64> {
    let mut ops = ops_tmp.clone();
    let mut res = Vec::new();

    println!("processing ops: {:?}", ops);

    let mut index = 0;
    while index < ops.len() {
        let mut value = ops[index];
        println!("execute {}", value);

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
                assert!(mc == 0);
                ops[c as usize] = get_value(&ops, a, ma) + get_value(&ops, b, mb);
                index += 4;
            } else if op == 2 {
                let a = ops[index + 1];
                let b = ops[index + 2];
                let c = ops[index + 3];
                assert!(mc == 0);
                ops[c as usize] = get_value(&ops, a, ma) * get_value(&ops, b, mb);
                index += 4;
            } else if op == 3 {
                let a = ops[index + 1];
                assert!(ma == 0);
                // ops[a as usize] = get_value(&ops, input, 0);
                ops[a as usize] = input;

                index += 2;
            } else if op == 4 {
                let a = ops[index + 1];
                let out = get_value(&ops, a, ma);
                res.push(out);
                println!("get output: {}", out);
                index += 2;
            } else if op == 5 {
                let a = ops[index + 1];
                let b = ops[index + 2];
                if get_value(&ops, a, ma) != 0 {
                    index = get_value(&ops, b, mb) as usize;
                } else {
                    index += 3;
                }
            } else if op == 6 {
                let a = ops[index + 1];
                let b = ops[index + 2];
                if get_value(&ops, a, ma) == 0 {
                    index = get_value(&ops, b, mb) as usize;
                } else {
                    index += 3;
                }
            } else if op == 7 {
                let a = ops[index + 1];
                let b = ops[index + 2];
                let c = ops[index + 3];
                assert!(mc == 0);
                if get_value(&ops, a, ma) < get_value(&ops, b, mb) {
                    ops[c as usize] = 1;
                } else {
                    ops[c as usize] = 0;
                }
                index += 4;

            } else if op == 8 {
                let a = ops[index + 1];
                let b = ops[index + 2];
                let c = ops[index + 3];
                assert!(mc == 0);
                if get_value(&ops, a, ma) == get_value(&ops, b, mb) {
                    ops[c as usize] = 1;
                } else {
                    ops[c as usize] = 0;
                }
                index += 4;
            } else {
                println!("Unknown op: {}", op);
                assert!(false);
            }
        }
    }
    return res;
}

fn main() {
    let filename = "in.txt";
    // let filename = "jmp1.txt";

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

    let INPUT = 5;
    let res = process_ops(ops.clone(), INPUT);

    if res.len() == 1 {
        println!("GOOD");
        println!("ans = {}", res[0]);
    } else {
        println!("NOT GOOD");
    }


}
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;


fn process_ops(ops_tmp: Vec<i64>, noun: i64, verb: i64) -> i64 {
    let mut ops = ops_tmp.clone();
    ops[1] = noun;
    ops[2] = verb;

    println!("processing ops: {:?}", ops);

    let mut index = 0;
    while index < ops.len() {
        let op = ops[index];
        if op == 99 {
            break;
        } else {
            let a = ops[index + 1] as usize;
            let b = ops[index + 2] as usize;
            let c = ops[index + 3] as usize;
            if op == 1 {
                ops[c] = ops[a] + ops[b];
                index += 4;
            } else if op == 2 {
                ops[c] = ops[a] * ops[b];
                index += 4;
            } else {
                println!("Unknown op: {}", op);
                assert!(false);
            }
        }
    }
    println!("at 0: {}", ops[0]);
    return ops[0];
}

fn main() {
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

    for noun in 0..100 {
        for verb in 0..100 {
            let ans = process_ops(ops.clone(), noun, verb);
            if ans == 19690720 {
                println!("{}, {} --> {}", noun, verb, 100 * noun + verb);
                process::exit(0);
            }
        }
    }

}
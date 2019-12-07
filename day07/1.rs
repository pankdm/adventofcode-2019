use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;
use std::collections::{HashMap, HashSet};


fn get_value(ops: &Vec<i64>, value: i64, mode: i64) -> i64 {
    if mode == 0 {
        return ops[value as usize];
    } else if mode == 1 {
        return value;
    }
    assert!(false);
    return -1;
}


fn process_ops(ops_tmp: Vec<i64>, input: Vec<i64>) -> Vec<i64> {
    let mut ops = ops_tmp.clone();
    let mut res = Vec::new();
    let mut input_ind = 0;
    assert!(input.len() == 2);

    // println!("processing ops: {:?}", ops);

    let mut index = 0;
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
                ops[a as usize] = input[input_ind];
                input_ind += 1;

                index += 2;
            } else if op == 4 {
                let a = ops[index + 1];
                let out = get_value(&ops, a, ma);
                res.push(out);
                // println!("get output: {}", out);
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

fn run_ampl(ops: Vec<i64>, phase: Vec<i64>) -> i64 {
    let mut prev_value = 0;
    let mut ans = 0;
    for i in 0..5 {
        let mut input = Vec::new();
        input.push(phase[i]);
        input.push(prev_value);

        let vals = process_ops(ops.clone(), input);
        assert!(vals.len() == 1);
        prev_value = vals[0];
        // ans *= 10;
        // ans += prev_value;
    }

    return prev_value;
}


fn main() {
    let filename = "in.txt";
    // let filename = "test1.txt";

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


    let mut ans = -1;

    for a in 0..5 {
        for b in 0..5 {
            for c in 0..5 {
                for d in 0..5 {
                    for e in 0..5 {
                        let mut unique = HashSet::new();
                        unique.insert(a);
                        unique.insert(b);
                        unique.insert(c);
                        unique.insert(d);
                        unique.insert(e);
                        if unique.len() != 5 {
                            continue;
                        }
                        let mut phase = Vec::new();
                        phase.push(a);
                        phase.push(b);
                        phase.push(c);
                        phase.push(d);
                        phase.push(e);

                        let now = run_ampl(ops.clone(), phase.clone());
                        println!("ran = {}, at {:?}", now, &phase);

                        if ans == -1 || now > ans {
                            ans = now;
                            println!("found better = {}, at {:?}", now, &phase);
                        }
                    }
                }
            }


        }
    }




    println!("ans = {}", ans);
}
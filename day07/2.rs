use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;
use std::collections::{HashMap, HashSet};
use std::collections::VecDeque;

fn get_value(ops: &Vec<i64>, value: i64, mode: i64) -> i64 {
    if mode == 0 {
        return ops[value as usize];
    } else if mode == 1 {
        return value;
    }
    assert!(false);
    return -1;
}




fn process_ops_until_output(ops: &mut Vec<i64>, index_: &mut Vec<usize>, input: &mut VecDeque<i64>,
    i : usize, halted: &mut bool) -> i64 {
    // let mut ops = ops_tmp.clone();
    // let mut res = Vec::new()
    // let mut input_ind = 0;
    // assert!(input.len() == 2);

    // let mut index = *index_;
    // let ref mut ops = ops_[i];
    // println!("processing ops: {:?}", ops);

    let mut index = index_[i];

    loop {
        let mut value = ops[index];
        println!("   >> index = {}, execute {}", index, value);

        let op = value % 100;
        value /= 100;

        let ma = value % 10;
        value /= 10;

        let mb = value % 10;
        value /= 10;

        let mc = value % 10;

        if op == 99 {
            *halted = true;
            println!("halted");
            return 0;
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
                ops[a as usize] = input.pop_front().unwrap();

                index += 2;
            } else if op == 4 {
                let a = ops[index + 1];
                let out = get_value(&ops, a, ma);
                // res.push(out);
                // println!("get output: {}", out);
                index += 2;
                println!("finished with index = {}", index);
                index_[i] = index;
                return out;
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
    assert!(false);
    return -1;
}

fn run_ampl(ops: Vec<i64>, phase: Vec<i64>) -> i64 {
    let mut prev_value = 0;
    let mut ans = 0;
    let mut ops_array = Vec::new();
    let mut index_array = Vec::new();
    let mut input_array = Vec::new();
    for i in 0..5 {
        ops_array.push(ops.clone());
        index_array.push(0 as usize);
        input_array.push(VecDeque::new());
        input_array[i].push_back(phase[i]);
    }

    let mut counter = 0;
    let mut e_value = 0;
    loop {
        let i = counter % 5;
        input_array[i].push_back(prev_value);

        let mut halted = false;
        let start_index = index_array[i];
        println!("step {}, starting from index {}, input size = {}", counter, start_index, input_array[i].len());
        let val = process_ops_until_output(&mut ops_array[i], &mut index_array, &mut input_array[i], i, &mut halted);
            println!("  ---> got: {}", val);

        if halted {
            break;
        }
        if i == 4 {
            e_value = val;
            println!("at iter {} produced {}", counter, e_value);
        }

        prev_value = val;
        // ans *= 10;
        // ans += prev_value;
        counter += 1;
        if counter > 200 {
            break
        }
    }

    return e_value;
}


fn main() {
    let filename = "in.txt";
    // let filename = "test2-1.txt";
    // let filename = "test2-2.txt";

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

    for a in 5..10 {
        for b in 5..10 {
            for c in 5..10 {
                for d in 5..10 {
                    for e in 5..10 {
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

                        // phase = vec![9,8,7,6,5];
                        // phase = vec![9,7,8,5,6];

                        let now = run_ampl(ops.clone(), phase.clone());
                        println!("ran = {}, at {:?}", now, &phase);

                        if ans == -1 || now > ans {
                            ans = now;
                            println!("found better = {}, at {:?}", now, &phase);
                        }
                        // process::exit(0);
                    }
                }
            }


        }
    }




    println!("ans = {}", ans);
}
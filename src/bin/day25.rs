use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{self, Write};
use std::io::{BufRead, BufReader};
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};
use std::env;


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
                if input.len() == 0 {
                    vm.index = index;
                    vm.base = base;
                    return res;
                }
                // if read_input {
                //     vm.index = index;
                //     vm.base = base;
                //     return res;
                // }
                // read_input = true;
                // assert!(ma == 0);
                // ops[a as usize] = get_value(&ops, input, 0);
                assert!(input.len() > 0);
                let pos = get_pos(a, ma, base);
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


fn send_command(vm: &mut Vm, cmd: &String) -> Vec<String> {
    // let mut vm = _vm.clone();
    let mut input = VecDeque::new();

    for c in cmd.chars() {
        input.push_back(c as u8 as i64);
    }
    input.push_back('\n' as i64);

    let res = process_ops(vm, &mut input);
    let mut ans = Vec::new();
    let mut s = "".to_string();
    for c in res {
        if c == '\n' as u8 as i64 {
            ans.push(s);
            s = "".to_string();
        } else {
            s.push(c as u8 as char);
        }
    }
    ans
}

fn find_message(output: &Vec<String>) -> bool {
    let message = "Alert! Droids on this ship are";
    for line in output {
        if line.find(message).is_some() {
            return true;
        }
    }
    return false;
}

fn show_items(output: &Vec<String>) {
    let mut inside_items = false;
    for line in output {
        if inside_items {
            if line.len() > 2 {
                if &line[0..1] == "-" {
                    let item = line[2..].to_string();
                    println!("item: {}", item);
                } else {
                    inside_items = false;
                }
            } else {
                inside_items = false;
            }
        }

        if line.to_string() == "Items here:".to_string() {
            inside_items = true;
        }
    }
}

fn get_doors(output: &Vec<String>) -> Vec<String> {
    let mut res = Vec::new();
    let mut inside_doors = false;
    for line in output {
        if inside_doors {
            if line.len() > 2 {
                if &line[0..1] == "-" {
                    let door = line[2..].to_string();
                    // println!("item: {}", item);
                    res.push(door);
                } else {
                    inside_doors = false;
                }
            } else {
                inside_doors = false;
            }
        }

        if line.to_string() == "Doors here lead:".to_string() {
            inside_doors = true;
        }
    }    
    res
}

fn get_location(output: &Vec<String>) -> String {
    for line in output {
        if line.len() > 2 && &line[0..2] == "==" {
            let ln = line.len();
            return line[3..ln - 3].to_string();
        }
    }   
    return "".to_string();
}

fn try_drops(_vm: &Vm) -> (i32, Vec<String>) {
    let mut vm = _vm.clone();
    let mut items = Vec::new();
    let inv_lines = send_command(&mut vm, &"inv".to_string());
    println!("got inv_lines = {:?}", inv_lines);
    for line in &inv_lines {
        if line.len() > 2 && line[0..1].to_string() == "-".to_string() {
            items.push(line[2..].to_string());
        }
    }
    println!("got {} items: {:?}", items.len(), items);
    let num = items.len();

    let mut file = File::create("log").unwrap();

    for mask in 0..=(1 << num) {
        // println!("trying mask {}", mask);

        let mut vm_new = vm.clone();
        let output = try_drop(&mut vm_new, mask, &items);
        if !find_message(&output) {
            println!("trying mask {}", mask);
            for line in &output {
                println!("{}", line);
            }
            return (mask as i32, items);

        }

        file.write_all(format!("trying mask {}", mask).as_bytes());
        for line in output {
            file.write_all(line.as_bytes());
            file.write_all(b"\n");
        }
    }
    return (-1, items);
}


fn try_drop(vm: &mut Vm, mask: usize, items: &Vec<String>) -> Vec<String> {
    let mut output = Vec::new();
    for i in 0..items.len() {
        let bit = 1 << i;
        if mask & bit > 0 {
            let os = send_command(vm, &format!("drop {}", items[i]).to_string());
            for o in os {
                output.push(o);
            }
        }
    }
    let os = send_command(vm, &"north".to_string());
    for o in os {
        output.push(o);
    }
    output
}


struct MiniMap {
    x: i32,
    y: i32,
    grid: HashMap<(i32, i32), char>,
}


fn get_dxdy(door: &str) -> (i32, i32) {
    if door == "north" {
        (0, -1)
    } else if door == "south" {
        (0, 1)
    } else if door == "west" {
        (-1, 0)
    } else if door == "east" {
        (1, 0)
    } else {
        (0, 0)
    }
}

fn print_map(mini_map: &MiniMap) {
    let mut minx = 0;
    let mut maxx = 0;
    let mut miny = 0;
    let mut maxy = 0;
    for (k, v) in &mini_map.grid {
        let (x, y) = k;
        minx = minx.min(*x);
        miny = miny.min(*y);

        maxx = maxx.max(*x);
        maxy = maxy.max(*y);
    }

    println!("");
    for y in miny..=maxy {
        for x in minx..=maxx {
            if x == mini_map.x && y == mini_map.y {
                print!("X");
                continue;
            }
            match mini_map.grid.get(&(x, y)) {
                Some(c) => { print!("{}", c); },
                _ => { print!(" ")}
            }
        }
        println!("");
    }
    println!("");
}


fn make_step(vm: &mut Vm,
    action: &String,
    now_loc: &mut String,
    visited: &mut HashSet<(String, String)>,
    options: &mut HashSet<(String, String)>,
    mini_map: &mut MiniMap,
    file: &mut File,
    with_logging: bool,
    log: &mut File) {

    let line = action.clone();
    println!("    at {} got: {}", now_loc, line);
    file.write_all(line.as_bytes());
    file.write_all(b"\n");

    if now_loc.to_string() != "".to_string() {
        visited.insert((now_loc.to_string(), line.to_string()));
    }
    let output = send_command(vm, &line);
    show_items(&output);
    let location = get_location(&output);

    {
        let (dx, dy) = get_dxdy(&line);
        let nx = mini_map.x + 3 * dx;
        let ny = mini_map.y + 3 * dy;
        if location != "".to_string() {
            mini_map.x = nx;
            mini_map.y = ny;
        }
    }


    let doors = get_doors(&output);
    *now_loc = location.clone();
    for door in &doors {
        options.insert((location.to_string(), door.to_string()));
    }


    for c_dx in -1..=1 {
        for c_dy in -1..=1 {
            let key = (mini_map.x + c_dx, mini_map.y + c_dy);
            if c_dx == 0 && c_dy == 0 {
                mini_map.grid.entry(key).or_insert('.');
            }
            let mut found = false;
            for door in &doors {
                let (dx, dy) = get_dxdy(&door);
                let key2 = (mini_map.x + dx, mini_map.y + dy);
                if key2 == key {
                    found = true;
                    break;
                }
            }
            if found {
                mini_map.grid.entry(key).or_insert('.');
            } else {
                mini_map.grid.entry(key).or_insert('#');
            }
        }
    }

    if with_logging {
        for o in &output {
            log.write_all(o.as_bytes());
            log.write_all(b"\n");
        }
    } else {
        for o in &output {
            println!("{}", o);
        }
        for key in options.iter() {
            if !visited.contains(key) {
                // println!("Not visited: {:?}", key);
            }
        }
        print_map(&mini_map);
    }
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
            ops: ops.clone(),
            index: 0,
            base: 0,
    };

    let now = SystemTime::now();
    let ts = now.duration_since(UNIX_EPOCH).unwrap().as_secs();
    let mut file = File::create(format!("foo-{}.txt", ts)).unwrap();


    let mut log = File::create("log.txt").unwrap();

    let mut mini_map = MiniMap {
        x : 0,
        y : 0,
        grid: HashMap::new(),
    };


    let mut now_loc = "".to_string();
    let mut visited = HashSet::new();
    let mut options = HashSet::new();

    // let mut f = File::open("foo-empty.txt").unwrap();
    let mut f = File::open("foo.txt").unwrap();

    let ff = BufReader::new(&f);

    make_step(
        &mut vm,
        &"take".to_string(),
        &mut now_loc,
        &mut visited,
        &mut options,
        &mut mini_map,
        &mut file,
        false,
        &mut log);

    for line_ in ff.lines() {
        let line = line_.unwrap();
        make_step(
            &mut vm,
            &line.to_string(),
            &mut now_loc,
            &mut visited,
            &mut options,
            &mut mini_map,
            &mut file,
            false,
            &mut log);
    }

    let (mask, items) = try_drops(&vm);
    println!("found mask {}", mask);
    if mask != -1 {
        try_drop(&mut vm, mask as usize, &items);
    }


    // let mut file = File::create(format!("foo.txt", ts)).unwrap();

    loop {
        println!("enter input: ");
        let stdin = io::stdin();
        let line = stdin.lock().lines().next().unwrap().unwrap();
        make_step(
            &mut vm,
            &line.to_string(),
            &mut now_loc,
            &mut visited,
            &mut options,
            &mut mini_map,
            &mut file,
            false,
            &mut log);
    }
// Items in your inventory:
// - festive hat
// - space heater
// - loom
// - space law space brochure
// - molten lava
// - sand
// - photons
// - pointer
// - wreath
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
    let lines = read_input("day25/in.txt");

    println!("part1 = {}", part1(&lines));

    // let lines = read_input("day19/t0.txt");
    // println!("part2 = {}", part2_file(&lines));
}

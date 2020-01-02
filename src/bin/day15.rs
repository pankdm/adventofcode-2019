use std::collections::{HashMap, VecDeque};

extern crate adventofcode;

use adventofcode::*;

fn wrapper(vm: &mut Vm, value: i64) -> i64 {
    let mut input = VecDeque::new();
    input.push_back(value);
    let res = process_ops(vm, &mut input);
    assert!(res.len() == 1, "{:?}", res);
    return res[0];
}

fn find_oxygen(vm_start: &Vm) -> i64 {
    let mut dist = HashMap::new();
    let mut q = VecDeque::new();

    let mut dirs = vec![(1, -1, 0), (2, 1, 0), (3, 0, -1), (4, 0, 1)];

    q.push_back((vm_start.clone(), (0, 0), 0));
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
                return d + 1;
            }
            if status == 0 {
                dist.insert(pos, -1);
                continue;
            }
            q.push_back((new_vm, (x, y), d + 1));
        }
    }

    -1
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut vm = Vm::from_string(&lines[0]);

    let ans = find_oxygen(&vm);
    ans
}

fn print_map(dist: &HashMap<(i64, i64), i64>, x00: i64, y00: i64) {
    let x_min = dist.keys().map(|p| p.0).min().unwrap();
    let x_max = dist.keys().map(|p| p.0).max().unwrap();
    let y_min = dist.keys().map(|p| p.1).min().unwrap();
    let y_max = dist.keys().map(|p| p.1).max().unwrap();

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
        println!("");
    }
}

fn find_oxygen2(vm_start: &Vm) -> (i64, i64, Vm) {
    let mut dist = HashMap::new();
    let mut q = VecDeque::new();

    let mut dirs = vec![(1, -1, 0), (2, 1, 0), (3, 0, -1), (4, 0, 1)];

    q.push_back((vm_start.clone(), (0, 0), 0));
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
            q.push_back((new_vm, (x, y), d + 1));
        }
    }
    unreachable!();
}

fn find_time(vm_start: &Vm) -> i64 {
    let (x00, y00, vm_start) = find_oxygen2(vm_start);

    println!("  ");
    let mut dist = HashMap::new();
    let mut q = VecDeque::new();

    let mut dirs = vec![(1, -1, 0), (2, 1, 0), (3, 0, -1), (4, 0, 1)];

    q.push_back((vm_start.clone(), (x00, y00), 0));
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
            q.push_back((new_vm, (x, y), d + 1));
        }
    }

    let mut best = 0;
    for (k, v) in dist.iter() {
        best = best.max(*v);
    }

    print_map(&dist, x00, y00);
    return best;
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut vm = Vm::from_string(&lines[0]);
    let ans = find_time(&vm);
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_input("day15/in.txt");
        assert_eq!(part1(&lines), 234);
    }

    #[test]
    fn test_part2() {
        let lines = read_input("day15/in.txt");
        assert_eq!(part2(&lines), 292);
    }
}

fn main() {
    let lines = read_input("day15/in.txt");

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}

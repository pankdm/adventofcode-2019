use std::collections::VecDeque;

extern crate adventofcode;

use adventofcode::*;
use permutohedron::LexicalPermutation;

fn run_ampl(vm_start: &Vm, phase: Vec<i64>) -> i64 {
    let mut prev_value = 0;
    for i in 0..5 {
        let mut input = VecDeque::new();
        input.push_back(phase[i]);
        input.push_back(prev_value);

        let mut vm = vm_start.clone();
        let vals = process_ops(&mut vm, &mut input);
        assert!(vals.len() == 1);
        prev_value = vals[0];
    }

    return prev_value;
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut vm = Vm::from_string(&lines[0]);

    let mut ans = -1;
    let mut data = [0, 1, 2, 3, 4];
    loop {
        let phase = data.to_vec();
        let now = run_ampl(&vm, phase);
        if ans == -1 || now > ans {
            ans = now;
        }
        if !data.next_permutation() {
            break;
        }
    }
    ans
}


fn run_ampl2(vm_start: &Vm, phase: Vec<i64>) -> i64 {
    let mut prev_value = 0;

    let mut vms = Vec::new();
    let mut inputs = Vec::new();

    for i in 0..5 {
        let vm = vm_start.clone();
        vms.push(vm);

        let mut input = VecDeque::new();
        input.push_back(phase[i]);
        inputs.push(input);
    }

    let mut counter = 0;
    let mut e_value = 0;
    loop {
        let i = counter % 5;
        inputs[i].push_back(prev_value);

        let vals = process_ops(&mut vms[i], &mut inputs[i]);
        assert_eq!(vals.len(), 1);
        let val = vals[0];

        if i == 4 {
            e_value = val;
            // println!("at iter {} produced {}", counter, e_value);
        }

        if i == 4 && vms[i].halted {
            break;
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


pub fn part2(lines: &Vec<String>) -> i64 {
    let mut vm = Vm::from_string(&lines[0]);

    let mut ans = -1;
    let mut data = [5, 6, 7, 8, 9];
    loop {
        let phase = data.to_vec();
        let now = run_ampl2(&vm, phase);
        if ans == -1 || now > ans {
            ans = now;
        }
        if !data.next_permutation() {
            break;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_input("day7/in.txt");
        assert_eq!(part1(&lines), 22012);
    }

    #[test]
    fn test_part2() {
        let lines = read_input("day7/in.txt");
        assert_eq!(part2(&lines), 4039164);
    }
}

fn main() {
    let lines = read_input("day7/in.txt");

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}

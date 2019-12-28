use std::collections::VecDeque;

extern crate adventofcode;

use adventofcode::*;

fn run_vm(vm: &mut Vm, noun: i64, verb: i64) -> i64 {
    vm.ops[1] = noun;
    vm.ops[2] = verb;

    let mut input: VecDeque<i64> = VecDeque::new();
    process_ops(vm, &mut input);

    vm.ops[0]
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut vm = Vm::from_string(&lines[0]);
    run_vm(&mut vm, 12, 2)
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let vm_start = Vm::from_string(&lines[0]);

    for noun in 0..100 {
        for verb in 0..100 {
            let mut vm = vm_start.clone();
            let ans = run_vm(&mut vm, noun, verb);
            if ans == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_input("day2/in.txt");
        assert_eq!(part1(&lines), 3101844);
    }

    #[test]
    fn test_part2() {
        let lines = read_input("day2/in.txt");
        assert_eq!(part2(&lines), 8478);
    }
}

fn main() {
    let lines = read_input("day2/in.txt");

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}

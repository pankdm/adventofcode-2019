use std::collections::VecDeque;

extern crate adventofcode;

use adventofcode::*;


fn get_diagnostics_code(res: &Vec<i64>) -> i64 {
    let mut non_zero_count = 0;
    let mut ans = 0;
    for i in 0..res.len() {
        if res[i] != 0 {
            non_zero_count += 1;
            ans = res[i];
        }
    }
    assert_eq!(non_zero_count, 1);
    ans
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut vm = Vm::from_string(&lines[0]);
    let mut input = VecDeque::new();
    input.push_back(1);

    let res = process_ops(&mut vm, &mut input);
    get_diagnostics_code(&res)
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut vm = Vm::from_string(&lines[0]);
    let mut input = VecDeque::new();
    input.push_back(5);

    let res = process_ops(&mut vm, &mut input);
    get_diagnostics_code(&res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_input("day5/in.txt");
        assert_eq!(part1(&lines), 13933662);
    }

    #[test]
    fn test_part2() {
        let lines = read_input("day5/in.txt");
        assert_eq!(part2(&lines), 2369720);
    }
}

fn main() {
    let lines = read_input("day5/in.txt");

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}

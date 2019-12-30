use std::collections::VecDeque;

extern crate adventofcode;

use adventofcode::*;


pub fn part1(lines: &Vec<String>) -> i64 {
    let mut vm = Vm::from_string(&lines[0]);
    let mut input = VecDeque::new();

    input.push_back(1);
    
    let res = process_ops(&mut vm, &mut input);
    assert_eq!(res.len(), 1);
    res[0]
}


pub fn part2(lines: &Vec<String>) -> i64 {
    let mut vm = Vm::from_string(&lines[0]);
    let mut input = VecDeque::new();

    input.push_back(2);
    
    let res = process_ops(&mut vm, &mut input);
    assert_eq!(res.len(), 1);
    res[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_input("day9/in.txt");
        assert_eq!(part1(&lines), 2955820355);
    }

    #[test]
    fn test_part2() {
        let lines = read_input("day9/in.txt");
        assert_eq!(part2(&lines), 46643);
    }
}

fn main() {
    let lines = read_input("day9/in.txt");

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}

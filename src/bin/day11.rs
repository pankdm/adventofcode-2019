use std::collections::{HashMap, VecDeque};

extern crate adventofcode;

use adventofcode::*;


type Grid = HashMap<(i64, i64), i64>;

fn walk_robot(vm_start: &Vm, start_color: i64) -> Grid {
    let mut vm = vm_start.clone();
    let mut input = VecDeque::new();

    let mut grid = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    grid.insert((x, y), start_color);
    let mut dx = 0;
    let mut dy = -1;
    let mut halted = false;
    let mut index = 0 as usize;
    let mut base = 0 as i64;
    loop {
        let key = (x, y);
        let mut color = 0;
        match grid.get(&key) {
            Some(&value) => color = value,
            _ => {}
        }
        input.push_back(color);
        let res = process_ops(&mut vm, &mut input);
        if vm.halted {
            break;
        }
        grid.insert(key, res[0]);
        if res[1] == 1 {
            let dytmp = dy;
            let dxtmp = dx;
            dx = dytmp;
            dy = -dxtmp;
        } else {
            let dytmp = dy;
            let dxtmp = dx;
            dx = -dytmp;
            dy = dxtmp;
        }
        x += dx;
        y += dy;
    }
    grid
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut vm = Vm::from_string(&lines[0]);
    let grid = walk_robot(&vm, 0);
    grid.len() as i64
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut vm = Vm::from_string(&lines[0]);
    let grid = walk_robot(&vm, 1);

    let minx = grid.keys().map(|p| { p.0 }).min().unwrap();
    let maxx = grid.keys().map(|p| { p.0 }).max().unwrap();
    let miny = grid.keys().map(|p| { p.1 }).min().unwrap();
    let maxy = grid.keys().map(|p| { p.1 }).max().unwrap();

    for y in miny..=maxy {
        for x in (minx..=maxx).rev() {
            let key = (x, y);
            let color = grid.get(&key).unwrap_or(&0);
            if *color == 0 {
                print!(" ");
            } else {
                print!("#");
            }
        }
        println!("");
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_input("day11/in.txt");
        assert_eq!(part1(&lines), 2093);
    }

    #[test]
    fn test_part2() {
        let lines = read_input("day11/in.txt");
        // assert_eq!(part2(&lines), 46643);
    }
}

fn main() {
    let lines = read_input("day11/in.txt");

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}

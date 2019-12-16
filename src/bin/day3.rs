extern crate adventofcode;
use adventofcode::*;

use std::collections::HashMap;

fn follow_line(line: &String) -> HashMap<(i64, i64), i64> {
    let mut grid = HashMap::new();

    let mut x = 0;
    let mut y = 0;

    let steps = line.split(",").collect::<Vec<&str>>();
    let mut counter = 0;
    for step in steps {
        let mut dx = 0;
        let mut dy = 0;
        let t = step.chars().next().unwrap();
        if t == 'R' {
            dx = 1;
        } else if t == 'L' {
            dx = -1;
        } else if t == 'U' {
            dy = 1;
        } else if t == 'D' {
            dy = -1;
        } else {
            assert!(false, "unknown step: {}", t);
        }
        let slice = &step[1..];
        let num = parse_i64(&slice.to_string());
        for _ in 0..num {
            x += dx;
            y += dy;
            counter += 1;
            grid.insert((x, y), counter);
        }
    }
    return grid;
}

pub fn part1(lines: &Vec<String>) -> i64 {
    assert_eq!(lines.len(), 2);
    let grid1 = follow_line(&lines[0]);
    let grid2 = follow_line(&lines[1]);

    let mut best = -1;
    for (xy, _v1) in grid1 {
        match grid2.get(&xy) {
            Some(&_v2) => {
                let dist = xy.0.abs() + xy.1.abs();
                if best == -1 || dist < best {
                    best = dist;
                }
            }
            _ => {}
        }
    }
    return best;
}

pub fn part2(lines: &Vec<String>) -> i64 {
    assert_eq!(lines.len(), 2);
    let grid1 = follow_line(&lines[0]);
    let grid2 = follow_line(&lines[1]);

    let mut best = -1;
    for (xy, v1) in grid1 {
        match grid2.get(&xy) {
            Some(&v2) => {
                let dist = v1 + v2;
                if best == -1 || dist < best {
                    best = dist;
                }
            }
            _ => {}
        }
    }
    return best;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_input("day3/in.txt");
        assert_eq!(part1(&lines), 1225);
    }

    #[test]
    fn test_part2() {
        let lines = read_input("day3/in.txt");
        assert_eq!(part2(&lines), 107036);
    }
}

fn main() {
    let lines = read_input("day3/in.txt");

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}

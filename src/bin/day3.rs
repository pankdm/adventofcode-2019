extern crate adventofcode;
use adventofcode::*;

use std::collections::HashMap;

fn follow_line(line: &String) -> HashMap<(i64, i64), i64> {
    let mut grid = HashMap::new();

    let mut x = 0;
    let mut y = 0;

    let steps = split_string(line, ",");
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
    fn test_part1_example1() {
        let input = r#"
R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83
"#;
        assert_eq!(part1(&to_lines(input)), 159);
    }

    #[test]
    fn test_part1_example2() {
        let input = r#"
R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7
"#;
        assert_eq!(part1(&to_lines(input)), 135);
    }


    #[test]
    fn test_part2_example1() {
        let input = r#"
R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83
"#;
        assert_eq!(part2(&to_lines(input)), 610);
    }

    #[test]
    fn test_part2_example2() {
        let input = r#"
R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7
"#.trim();
        assert_eq!(part2(&to_lines(input)), 410);
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

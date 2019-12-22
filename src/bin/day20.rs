extern crate adventofcode;
use adventofcode::*;

use std::collections::BTreeSet;
use std::collections::{HashMap, HashSet, VecDeque};


fn is_letter(c: char) -> bool {
    'A' <= c && c <= 'Z'
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut grid = Vec::new();

    for l in lines {
        let mut row = Vec::new();
        for ch in l.chars() {
            row.push(ch);
        }
        grid.push(row);
    }

    let mut portals = HashMap::new();

    for y in 0..(grid.len() - 1) {
        for x in 0..(grid[y].len() - 1) {
            let now = grid[y][x];
            let right = grid[y][x + 1];
            let down = grid[y + 1][x];
            if is_letter(now) && is_letter(right) {
                let mut s = "".to_string();
                s.push(now);
                s.push(right);
                if x + 2 < grid[y].len() && grid[y][x + 2] == '.' {
                    let pt = ((x + 2) as i64, y as i64);
                    portals.entry(s.clone()).or_insert(Vec::new()).push(pt);
                }
                if x > 0 && grid[y][x - 1] == '.' {
                    let pt = ((x - 1) as i64, y as i64);
                    portals.entry(s).or_insert(Vec::new()).push(pt);
                } 
            }
            if is_letter(now) && is_letter(down) {
                let mut s = "".to_string();
                s.push(now);
                s.push(down);
                if y + 2 < grid.len() && grid[y + 2][x] == '.' {
                    let pt = (x as i64, (y + 2) as i64);
                    portals.entry(s.clone()).or_insert(Vec::new()).push(pt);
                }
                if y > 0 && grid[y - 1][x] == '.' {
                    let pt = (x as i64, (y - 1) as i64);
                    portals.entry(s).or_insert(Vec::new()).push(pt);
                }                
            }
        }
    }

    let mut next = HashMap::new();

    let mut start:(i64, i64) = (-1, -1);
    let mut end:(i64, i64) = (-1, -1);

    for (k, values) in portals.clone() {
        println!("{} is at {:?}", k, values);
        if k == "AA" {
            assert!(values.len() == 1);
            start = values[0];
            continue;
        }
        if k == "ZZ" {
            assert!(values.len() == 1);
            end = values[0];
            continue;
        }
        assert!(values.len() == 2);
        let v0 = values[0];
        let v1 = values[1];
        next.insert(v0, v1);
        next.insert(v1, v0);
    }

    let mut q = VecDeque::new();

    q.push_back((start, 0));
    let mut visited = HashSet::new();

    let mut dirs: Vec<(i64, i64)> = vec![(-1, 0), (1, 0), (0, 1), (0, -1)];

    while !q.is_empty() {
        let (now, d) = q.pop_front().unwrap();
        if now == end {
            return d;
        }
        let (cx, cy) = now;
        println!("at {:?} d = {}, ch = {}", now, d, grid[cy as usize][cx as usize]);


        for (dx, dy) in &dirs {
            let mut nx = cx + dx;
            let mut ny = cy + dy;
            let mut key = (nx, ny);
            let ch = grid[ny as usize][nx as usize];
            if ch == '#' {
                continue;
            }
            if is_letter(ch) {
                if now == start {
                    continue;
                }
                // println!("  ch = {}, finding next for {:?}", ch, &(nx, ny));
                key = next[&(cx, cy)];
                // println!("   found {:?}", &key);
            }
            if visited.contains(&key) {
                continue;
            }
            q.push_back((key, d + 1));
            visited.insert(key);
        }
    }
    unreachable!();
}

fn is_outer(key: (i64, i64), bounds: (i64, i64)) -> bool {
    let (x, y) = key;
    let (mx, my) = bounds;
    x == 2 || y == 2 || x + 3 == mx || y + 3 == my
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut grid = Vec::new();

    for l in lines {
        let mut row = Vec::new();
        for ch in l.chars() {
            row.push(ch);
        }
        grid.push(row);
    }

    let mut portals = HashMap::new();

    for y in 0..(grid.len() - 1) {
        for x in 0..(grid[y].len() - 1) {
            let now = grid[y][x];
            let right = grid[y][x + 1];
            let down = grid[y + 1][x];
            if is_letter(now) && is_letter(right) {
                let mut s = "".to_string();
                s.push(now);
                s.push(right);
                if x + 2 < grid[y].len() && grid[y][x + 2] == '.' {
                    let pt = ((x + 2) as i64, y as i64);
                    portals.entry(s.clone()).or_insert(Vec::new()).push(pt);
                }
                if x > 0 && grid[y][x - 1] == '.' {
                    let pt = ((x - 1) as i64, y as i64);
                    portals.entry(s).or_insert(Vec::new()).push(pt);
                } 
            }
            if is_letter(now) && is_letter(down) {
                let mut s = "".to_string();
                s.push(now);
                s.push(down);
                if y + 2 < grid.len() && grid[y + 2][x] == '.' {
                    let pt = (x as i64, (y + 2) as i64);
                    portals.entry(s.clone()).or_insert(Vec::new()).push(pt);
                }
                if y > 0 && grid[y - 1][x] == '.' {
                    let pt = (x as i64, (y - 1) as i64);
                    portals.entry(s).or_insert(Vec::new()).push(pt);
                }                
            }
        }
    }

    let mut next = HashMap::new();

    let mut start:(i64, i64) = (-1, -1);
    let mut end:(i64, i64) = (-1, -1);

    let bounds = (grid[0].len() as i64, grid.len() as i64);


    for (k, values) in portals.clone() {
        println!("{} is at {:?}", k, values);
        if k == "AA" {
            assert!(values.len() == 1);
            start = values[0];
            continue;
        }
        if k == "ZZ" {
            assert!(values.len() == 1);
            end = values[0];
            continue;
        }
        assert!(values.len() == 2);
        let v0 = values[0];
        let v1 = values[1];
        println!("  added connect {:?} ({}) -> {:?} ({})", v0, is_outer(v0, bounds), v1, is_outer(v1, bounds));
        next.insert(v0, v1);
        next.insert(v1, v0);
    }

    let mut q = VecDeque::new();

    q.push_back((start, 0, 0));
    let mut visited = HashSet::new();

    let mut dirs: Vec<(i64, i64)> = vec![(-1, 0), (1, 0), (0, 1), (0, -1)];

    while !q.is_empty() {
        let (now, level, d) = q.pop_front().unwrap();
        // if now == end {
        //     return d;
        // }
        let (cx, cy) = now;
        // println!("at {:?} lvl = {}, d = {}, ch = {}", now, level, d, grid[cy as usize][cx as usize]);


        for (dx, dy) in &dirs {
            let mut nx = cx + dx;
            let mut ny = cy + dy;
            let mut key = (nx, ny);
            let mut next_level = level;
            let ch = grid[ny as usize][nx as usize];
            if ch == '#' {
                continue;
            }
            if is_letter(ch) {
                if now == start {
                    continue;
                }
                if now == end {
                    if next_level == 0 {
                        return d;
                    } else {
                        continue;
                    }
                }
                // println!("  ch = {}, finding next for {:?}", ch, &(nx, ny));
                key = next[&now];
                // println!("  teleporting to {:?} at {}", key, ch);
                if is_outer(now, bounds) {
                    if next_level == 0 {
                        continue;
                    }
                    next_level -= 1;
                } else {
                    next_level += 1;
                }
                // println!("   found {:?}", &key);
            }
            let state = (key, next_level);
            if visited.contains(&state) {
                continue;
            }
            q.push_back((key, next_level, d + 1));
            visited.insert(state);
        }
    }
    unreachable!();
}




#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_part1() {
    //     let lines = read_input("day18/in.txt");
    //     assert_eq!(part1(&lines), 7430);
    // }

    // #[test]
    // fn test_part2() {
    //     let lines = read_input("day18/in.txt");
    //     assert_eq!(part2(&lines), 1864);
    // }
}

fn main() {
    // let lines = read_input("day16/t2-t0.txt");

    // let lines = read_input("day18/t2.txt");
    // let lines = read_input("day20/t0.txt");
    // let lines = read_input("day20/t1.txt");

    let lines = read_input("day20/in.txt");

    // println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));

    // println!("part2 = {}", part2_dummy(&lines));
}

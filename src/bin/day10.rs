use std::collections::{HashMap, HashSet};
use std::f64;
use std::fs::File;
use std::io::{BufRead, BufReader};

extern crate adventofcode;
use adventofcode::*;

fn gcd(a: i64, b: i64) -> i64 {
    if a < b {
        return gcd(b, a);
    }
    if b == 0 {
        return a;
    }
    return gcd(a - b, b);
}

fn count_asteroid(grid: &Vec<String>, x0: i64, y0: i64) -> i64 {
    let mut count = 0;
    let mut unique = HashSet::new();
    for _y in 0..grid.len() {
        for _x in 0..grid[_y].len() {
            let x = _x as i64;
            let y = _y as i64;
            if x == x0 && y == y0 {
                // count += 1;
                continue;
            }
            if grid[_y].as_bytes()[_x] as char == '#' {
                let dx = x - x0;
                let dy = y - y0;
                let d = gcd(dx.abs(), dy.abs());
                assert!(dx % d == 0);
                assert!(dy % d == 0);
                let key = (dx / d, dy / d);
                if !unique.contains(&key) {
                    count += 1;
                // println!("{:?} was not found before", &key);
                } else {
                    // println!("  {:?} is already there", &key);
                }
                unique.insert(key);
            }
        }
    }
    return count;
}

fn count_asteroid2(grid: &Vec<Vec<char>>, x0: i64, y0: i64) -> i64 {
    let mut count = 0;
    let mut unique = HashSet::new();
    for _y in 0..grid.len() {
        for _x in 0..grid[_y].len() {
            let x = _x as i64;
            let y = _y as i64;
            if x == x0 && y == y0 {
                // count += 1;
                continue;
            }
            if grid[_y][_x] as char == '#' {
                let dx = x - x0;
                let dy = y - y0;
                let d = gcd(dx.abs(), dy.abs());
                assert!(dx % d == 0);
                assert!(dy % d == 0);
                let key = (dx / d, dy / d);
                if !unique.contains(&key) {
                    count += 1;
                // println!("{:?} was not found before", &key);
                } else {
                    // println!("  {:?} is already there", &key);
                }
                unique.insert(key);
            }
        }
    }
    return count;
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut grid = Vec::new();
    for line in lines {
        grid.push(line.trim().to_string());
    }

    let mut best = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let count = count_asteroid(&grid, x as i64, y as i64);
            if count > best {
                best = count;
            }
        }
    }
    best
}

struct Asteroid {
    x: i64,
    y: i64,
    slope: (i64, i64),
    d: i64,
}

fn angle(slope: (i64, i64)) -> f64 {
    let x = slope.0 as f64;
    let y = slope.1 as f64;
    let res = y.atan2(x) + 0.5 * f64::consts::PI;
    if res < 0.0 {
        return res + 2.0 * f64::consts::PI;
    } else {
        return res;
    }
}

fn sort_asteroids(grid: &mut Vec<Vec<char>>, x0: i64, y0: i64) -> Vec<Asteroid> {
    let mut unique = HashMap::new();
    for _y in 0..grid.len() {
        for _x in 0..grid[_y].len() {
            let x = _x as i64;
            let y = _y as i64;
            if x == x0 && y == y0 {
                // count += 1;
                continue;
            }
            if grid[_y][_x] as char == '#' {
                let dx = x - x0;
                let dy = y - y0;
                let d = gcd(dx.abs(), dy.abs());
                assert!(dx % d == 0);
                assert!(dy % d == 0);
                let slope = (dx / d, dy / d);
                let ast = Asteroid { x, y, slope, d };
                if !unique.contains_key(&slope) {
                    unique.entry(slope).or_insert(ast);
                } else {
                    let ref other = unique.get(&slope).unwrap();
                    if ast.d < other.d {
                        unique.insert(slope, ast);
                    }
                }
            }
        }
    }

    let mut asts = Vec::new();
    for (key, ast) in unique {
        asts.push(ast);
    }

    println!("sorting");
    asts.sort_by(|l, r| {
        return angle(l.slope).partial_cmp(&angle(r.slope)).unwrap();
    });

    // for i in 0..10.min(asts.len()) {
    //     grid[asts[i].y as usize][asts[i].x as usize] = (i as u8 + '0' as u8) as char;
    // }
    // for y in 0..grid.len() {
    //     for x in 0..grid[y].len() {
    //         print!("{}", grid[y][x]);
    //     }
    //     print!("\n");
    //     io::stdout().flush().unwrap();
    // }
    return asts;
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut grid = Vec::new();
    for line in lines {
        let mut row = Vec::new();
        for ch in line.trim().chars() {
            row.push(ch);
        }
        grid.push(row);
    }

    let mut best = 0;
    let mut xbest = 0;
    let mut ybest = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] != '#' {
                continue;
            }
            let count = count_asteroid2(&grid, x as i64, y as i64);
            if count > best {
                best = count;
                xbest = x;
                ybest = y;
            }
        }
    }

    // let mut best = count_asteroid(&grid, 3, 4);
    // xbest = 5;
    // ybest = 3;
    println!("starting at {:?}, count = {}", (xbest, ybest), best);
    grid[ybest][xbest] = 'X';

    let mut nth = (200 as i64) - 1;
    loop {
        let asts = sort_asteroids(&mut grid, xbest as i64, ybest as i64);
        if nth < asts.len() as i64 {
            let ref ast = asts[nth as usize];
            let ans = ast.x * 100 + ast.y;
            // println!("ans = {}", ans);
            return ans;
        }
        if asts.len() == 0 {
            println!("nothing more to remove");
            return -1;
        }

        nth -= asts.len() as i64;
        for ast in &asts {
            grid[ast.y as usize][ast.x as usize] = '.';
        }
        println!("removed {} asts, left = {}", &asts.len(), nth);
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_input("day10/in.txt");
        assert_eq!(part1(&lines), 280);
    }

    #[test]
    fn test_part2() {
        let lines = read_input("day10/in.txt");
        assert_eq!(part2(&lines), 706);
    }
}

fn main() {
    let lines = read_input("day10/in.txt");

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}

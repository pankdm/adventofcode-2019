extern crate adventofcode;
use adventofcode::*;

use std::collections::BTreeSet;
use std::collections::{HashMap, HashSet, VecDeque};

type Grid = Vec<Vec<char>>;

type InfGrid = HashMap<i32, Grid>;

fn hash(grid: &Grid) -> i64 {
    let mut pow = 1 as i64;
    let mut res = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '#' {
                res += pow;
            }
            pow *= 2;
        }
    }
    res
}

fn get_tile(x: i64, y: i64, grid: &Grid) -> char {
    if y < 0 || x < 0 || y as usize >= grid.len() || x as usize >= grid[y as usize].len() {
        return '.';
    }
    return grid[y as usize][x as usize];
}

fn get_inf_tile(x: i64, y: i64, level: i32, inf_grid: &InfGrid) -> char {
    if !inf_grid.contains_key(&level) {
        return '.';
    }
    assert!(y >= 0);
    assert!(x >= 0);
    assert!((y as usize) < inf_grid[&level].len());
    assert!((x as usize) < inf_grid[&level][y as usize].len());

    let ch = inf_grid[&level][y as usize][x as usize];
    assert_ne!(ch, '?');
    ch
}


fn step_bugs(grid: &Grid) -> Grid {
    let mut next = grid.clone();

    let mut dirs = Vec::new();
    dirs.push((-1, 0));
    dirs.push((1, 0));
    dirs.push((0, -1));
    dirs.push((0, 1));

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let mut num_bugs = 0;
            for (dx, dy) in &dirs {
                let mut nx = x as i64 + dx;
                let mut ny = y as i64 + dy;
                if get_tile(nx, ny, grid) == '#' {
                    num_bugs += 1;
                }
            }
            if grid[y][x] == '.' {
                if num_bugs == 1 || num_bugs == 2 {
                    next[y][x] = '#';
                } else {
                    next[y][x] = '.';
                }
            } else {
                if num_bugs != 1 {
                    next[y][x] = '.';
                } else {
                    next[y][x] = '#';
                }
            }
        }
    }
    next
}

fn print_layout(grid: &Grid) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            print!("{}", grid[y][x]);
        }
        println!("");
    }
}


pub fn part1(lines: &Vec<String>) -> i64 {
    let mut grid = Vec::new();
    for l in lines {
        let mut row = Vec::new();
        for c in l.chars() {
            if c == '#' || c == '.' {
                row.push(c);
            }
        }
        grid.push(row);
    }

    let mut before = HashSet::new();

    let mut now = grid;

    let mut cnt = 0;
    loop {
        // println!("");
        // println!("cnt = {}", cnt);
        // print_layout(&now);

        let h = hash(&now);
        if before.contains(&h) {
            return h;
        }
        before.insert(h);

        let next = step_bugs(&now);
        now = next;

        cnt += 1;
    }
}


fn default_grid() -> Grid {
    let mut res = Vec::new();
    for y in 0..5 {
        let mut row = Vec::new();
        for x in 0..5 {
            row.push('.');
        }
        res.push(row);
    }
    res
}


fn get_adjacent(x: i64, y: i64, level: i32) -> Vec<(i64, i64, i32)> {
    let mut dirs = Vec::new();
    dirs.push((-1, 0));
    dirs.push((1, 0));
    dirs.push((0, -1));
    dirs.push((0, 1));

    let mut res = Vec::new();
    for (dx, dy) in dirs {
        let nx = x + dx;
        let ny = y + dy;
        if nx < 0 {
            res.push((1, 2, level - 1));
        } else if ny < 0 {
            res.push((2, 1, level - 1));
        } else if nx > 4 {
            res.push((3, 2, level - 1));
        } else if ny > 4 {
            res.push((2, 3, level - 1));
        } else if nx == 2 && ny == 2 {
            if dx == 1 {
                for iy in 0..5 {
                    res.push((0, iy, level + 1));
                }
            } else if dx == -1 {
                for iy in 0..5 {
                    res.push((4, iy, level + 1));
                }
            } else if dy == 1 {
                for ix in 0..5 {
                    res.push((ix, 0, level + 1));
                }
            } else if dy == -1 {
                for ix in 0..5 {
                    res.push((ix, 4, level + 1));
                }
            } else {
                unreachable!();
            }
        } else {
            res.push((nx, ny, level));
        }
    }
    res
}

fn step_inf(inf_grid: &InfGrid) ->  InfGrid {
    let mut next = inf_grid.clone();
    for (level, grid) in inf_grid {
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if x == 2 && y == 2 {
                    continue;
                }
                let mut num_bugs = 0;
                let adj = get_adjacent(x as i64, y as i64, *level);
                for (nx, ny, nlevel) in adj {
                    if get_inf_tile(nx, ny, nlevel, &inf_grid) == '#' {
                        num_bugs += 1;
                    }
                }
                let mut next_char = ' ';
                if grid[y][x] == '.' {
                    if num_bugs == 1 || num_bugs == 2 {
                        next_char = '#';
                    } else {
                        next_char = '.';
                    }
                } else {
                    if num_bugs != 1 {
                        next_char = '.';
                    } else {
                        next_char = '#';
                    }
                }
                next.get_mut(level).unwrap()[y][x] = next_char;
                // let mut next_grid = &mut next[&level];
                // next_grid[y][x] = next_char;
            }
        }
    }
    next
}


fn count_bugs(inf_grid: &InfGrid) -> i64 {
    let mut cnt = 0;
    for (level, grid) in inf_grid {
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == '#' {
                    cnt += 1;
                }
            }
        }
    }
    cnt
}

pub fn part2(lines: &Vec<String>, steps: i32) -> i64 {
    let mut grid = Vec::new();
    for l in lines {
        let mut row = Vec::new();
        for c in l.chars() {
            if c == '#' || c == '.' || c == '?' {
                row.push(c);
            }
        }
        grid.push(row);
    }
    print_layout(&grid);

    let mut inf_grid = HashMap::new();
    for i in -steps..=steps {
        if i == 0 {
            inf_grid.insert(0, grid.clone());
        } else {
            inf_grid.insert(i, default_grid());
        }
    }

    let mut bugs = 0;
    for i in 0..steps {
        // println!("");
        // println!("i = {}", i);
        // print_layout(&inf_grid[&0]);

        let next = step_inf(&inf_grid);
        inf_grid = next;
        bugs = count_bugs(&inf_grid);
        println!("  after {} steps, bugs = {}", i + 1, bugs);
    }
    bugs
}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_part1() {
//         let lines = read_input("day22/in.txt");
//         assert_eq!(part1(&lines), 3324);
//     }

//     #[test]
//     fn test_part2() {
//         let lines = read_input("day22/in.txt");
//         assert_eq!(part2(&lines), 74132511136410);
//     }
// }

fn main() {
    // let lines = read_input("day24/t1.txt");
    // let lines = read_input("day24/t0.txt");

    // println!("part1 = {}", part1(&lines));

    // let lines = read_input("day24/t1.txt");
    // println!("part2 = {}", part2(&lines, 10));

    let lines = read_input("day24/in.txt");
    println!("part2 = {}", part2(&lines, 200));

}

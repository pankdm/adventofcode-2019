extern crate adventofcode;
use adventofcode::*;

use std::collections::BTreeSet;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, Write};
use std::time::{Duration, Instant};

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut visited = HashMap::new();
    let mut q = VecDeque::new();

    let mut x0 = 0;
    let mut y0 = 0;

    let mut grid = Vec::new();

    let mut final_mask = 0 as i64;

    let mut cy = 0 as i64;
    for line in lines {
        let mut row = Vec::new();
        let mut cx = 0 as i64;
        for ch in line.chars() {
            row.push(ch);
            if ch == '@' {
                x0 = cx;
                y0 = cy;
            }
            if 'a' <= ch && ch <= 'z' {
                final_mask |= (1 << (ch as u8 - 'a' as u8) as i64);
            }
            cx += 1;
        }
        grid.push(row);
        cy += 1
    }

    let mut dirs = Vec::new();
    dirs.push((-1, 0));
    dirs.push((1, 0));
    dirs.push((0, -1));
    dirs.push((0, 1));

    q.push_back((x0, y0, 0, 0));
    while !q.is_empty() {
        let (xnow, ynow, mask_now, d) = q.pop_front().unwrap();
        if mask_now == final_mask {
            return d;
        }
        for (dx, dy) in dirs.clone() {
            let mut mask = mask_now;
            let x = xnow + dx;
            let y = ynow + dy;
            let mut is_ok = false;
            let ch = grid[y as usize][x as usize];
            if ch == '.' {
                is_ok = true;
            }
            if 'A' <= ch && ch <= 'Z' {
                let bit = 1 << (ch as u8 - 'A' as u8) as i64;
                if mask_now & bit > 0 {
                    is_ok = true;
                }
            }
            if 'a' <= ch && ch <= 'z' {
                let bit = 1 << (ch as u8 - 'a' as u8) as i64;
                mask |= bit;
                is_ok = true;
            }

            if !is_ok {
                continue;
            }

            let state = (x, y, mask);
            if visited.contains_key(&state) {
                continue;
            }
            visited.insert(state, d + 1);
            q.push_back((x, y, mask, d + 1));
        }
    }
    unreachable!();
}

// letter index, distance, required mask
type AllStates = Vec<(i64, i64, i64)>;
type Graph = HashMap<i64, AllStates>;
type Grid = Vec<Vec<char>>;
// index to (x, y)
type IndexMapping = HashMap<i64, (i64, i64)>;

fn visit_from(x0: i64, y0: i64, grid: &Grid) -> AllStates {
    let mut dirs = Vec::new();
    dirs.push((-1, 0));
    dirs.push((1, 0));
    dirs.push((0, -1));
    dirs.push((0, 1));

    let mut visited = HashMap::new();
    let mut q = VecDeque::new();

    q.push_back((x0, y0, 0, 0));

    let mut all_states = Vec::new();

    while !q.is_empty() {
        let (xnow, ynow, mask_now, d) = q.pop_front().unwrap();
        for (dx, dy) in dirs.clone() {
            let mut mask = mask_now;
            let x = xnow + dx;
            let y = ynow + dy;
            let ch = grid[y as usize][x as usize];
            if ch == '#' {
                continue;
            }
            if ch == '.' {}
            if 'A' <= ch && ch <= 'Z' {
                let bit = 1 << (ch as u8 - 'A' as u8) as i64;
                mask |= bit;
            }
            if 'a' <= ch && ch <= 'z' {
                let index = (ch as u8 - 'a' as u8) as i64;
                // println!("    at ")
                all_states.push((index, mask, d + 1));
            }

            let state = (x, y, mask);
            if visited.contains_key(&state) {
                continue;
            }
            visited.insert(state, d + 1);
            q.push_back((x, y, mask, d + 1));
        }
    }
    return all_states;
}

fn build_graph(x0: i64, y0: i64, grid: &Grid, mapping: &IndexMapping) -> Graph {
    let mut g = HashMap::new();
    let all_states = visit_from(x0, y0, grid);
    g.insert(-1, all_states.clone());

    let mut visited = HashSet::new();

    for (index, _, _) in all_states {
        if visited.contains(&index) {
            continue;
        }
        visited.insert(index);

        let (x, y) = mapping.get(&index).unwrap();
        println!(
            "  building reachable graph for {} from x={}, y={}",
            get_char(index),
            *x,
            *y
        );
        let index_states = visit_from(*x, *y, grid);
        g.insert(index, index_states);
    }
    g
}

fn get_char(index: i64) -> char {
    (index as u8 + 'a' as u8) as char
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut x0 = 0;
    let mut y0 = 0;

    let mut grid = Vec::new();

    let mut final_mask = 0 as i64;
    let mut mapping = HashMap::new();

    let mut cy = 0 as i64;
    for line in lines {
        let mut row = Vec::new();
        let mut cx = 0 as i64;
        for ch in line.chars() {
            row.push(ch);
            if ch == '@' {
                x0 = cx;
                y0 = cy;
            }
            if 'a' <= ch && ch <= 'z' {
                let index = (ch as u8 - 'a' as u8) as i64;
                let bit = (1 << index);
                assert!(final_mask & bit == 0);
                final_mask |= bit;
                mapping.insert(index, (cx, cy));
            }
            cx += 1;
        }
        grid.push(row);
        cy += 1
    }

    let mut dirs = Vec::new();
    dirs.push((-1, 0));
    dirs.push((1, 0));
    dirs.push((0, -1));
    dirs.push((0, 1));

    // let mut visited = HashMap::new();
    // let mut q = VecDeque::new();
    for iy in vec![y0 - 1, y0, y0 + 1] {
        for ix in vec![x0 - 1, x0, x0 + 1] {
            grid[iy as usize][ix as usize] = '#';
        }
    }

    for iy in vec![y0 - 1, y0 + 1] {
        for ix in vec![x0 - 1, x0 + 1] {
            grid[iy as usize][ix as usize] = '.';
        }
    }

    let mut graphs = Vec::new();

    for iy in vec![y0 - 1, y0 + 1] {
        for ix in vec![x0 - 1, x0 + 1] {
            println!("building graph from x={}, y={}", ix, iy);
            let g = build_graph(ix, iy, &grid, &mapping);
            graphs.push(g);
        }
    }

    let mut q = BTreeSet::new();
    let mut res = HashMap::new();

    // dijstra
    q.insert((0, (-1, -1, -1, -1, 0)));
    while !q.is_empty() {
        let now = q.iter().next().unwrap().clone();
        q.remove(&now);

        let (d_now, key_now) = now;
        let (i0, i1, i2, i3, mask_now) = key_now;
        let indexes = vec![i0, i1, i2, i3];

        // println!("d = {}, at state {:?}, mask = {}", d_now, indexes, mask_now);

        if mask_now == final_mask {
            return d_now;
        }

        for i in 0..4 {
            let current_node = indexes[i];
            for (index, mask_need, dist) in graphs[i][&current_node].iter() {
                // println!("for i = {} (node = {}), looking at dist = {}, index = {}, mask_need = {}",
                // i, current_node, dist, get_char(*index), mask_need);
                if mask_need | mask_now != mask_now {
                    continue;
                }
                let mut mask_next = mask_now | ((1 as i64) << index);
                let d_next = d_now + dist;
                let mut indexes_next = indexes.clone();
                indexes_next[i] = *index;
                let i0_next = indexes_next[0];
                let i1_next = indexes_next[1];
                let i2_next = indexes_next[2];
                let i3_next = indexes_next[3];

                let key = (i0_next, i1_next, i2_next, i3_next, mask_next);
                let state = (d_next, key);

                let mut need_update = false;
                match res.get(&key) {
                    Some(&d_old) => {
                        if d_next < d_old {
                            need_update = true;
                            q.remove(&(d_old, key));
                        }
                    }
                    _ => {
                        need_update = true;
                    }
                }

                if need_update {
                    res.insert(key, d_next);
                    q.insert(state);
                }
            }
        }
    }
    unreachable!();
}

pub fn bfs(x0: i64, y0: i64, grid: &Grid) -> i64 {
    let mut dirs = Vec::new();
    dirs.push((-1, 0));
    dirs.push((1, 0));
    dirs.push((0, -1));
    dirs.push((0, 1));

    let mut final_mask = 0;

    {
        let mut visited = HashMap::new();
        let mut q = VecDeque::new();
        q.push_back((x0, y0, 0, 0));
        while !q.is_empty() {
            let (xnow, ynow, mask_now, d) = q.pop_front().unwrap();
            // if mask_now == final_mask {
            //     return d;
            // }
            for (dx, dy) in dirs.clone() {
                let mut mask = mask_now;
                let x = xnow + dx;
                let y = ynow + dy;
                let mut is_ok = false;
                let ch = grid[y as usize][x as usize];
                if ch == '.' {
                    is_ok = true;
                }
                if 'A' <= ch && ch <= 'Z' {
                    // let bit = 1 << (ch as u8 - 'A' as u8) as i64;
                    is_ok = true;
                }
                if 'a' <= ch && ch <= 'z' {
                    let bit = 1 << (ch as u8 - 'a' as u8) as i64;
                    // mask |= bit;
                    final_mask |= bit;
                    is_ok = true;
                }

                if !is_ok {
                    continue;
                }

                let state = (x, y, mask);
                if visited.contains_key(&state) {
                    continue;
                }
                visited.insert(state, d + 1);
                q.push_back((x, y, mask, d + 1));
            }
        }
    }
    {
        let mut visited = HashMap::new();
        let mut q = VecDeque::new();
        q.push_back((x0, y0, 0, 0));
        while !q.is_empty() {
            let (xnow, ynow, mask_now, d) = q.pop_front().unwrap();
            if mask_now == final_mask {
                return d;
            }
            for (dx, dy) in dirs.clone() {
                let mut mask = mask_now;
                let x = xnow + dx;
                let y = ynow + dy;
                let mut is_ok = false;
                let ch = grid[y as usize][x as usize];
                if ch == '.' {
                    is_ok = true;
                }
                if 'A' <= ch && ch <= 'Z' {
                    let bit = 1 << (ch as u8 - 'A' as u8) as i64;
                    if final_mask & bit == 0 {
                        is_ok = true;
                    }
                    if mask_now & bit > 0 {
                        is_ok = true;
                    }
                }
                if 'a' <= ch && ch <= 'z' {
                    let bit = 1 << (ch as u8 - 'a' as u8) as i64;
                    mask |= bit;
                    is_ok = true;
                }

                if !is_ok {
                    continue;
                }

                let state = (x, y, mask);
                if visited.contains_key(&state) {
                    continue;
                }
                visited.insert(state, d + 1);
                q.push_back((x, y, mask, d + 1));
            }
        }
        unreachable!();
    }
}

pub fn part2_dummy(lines: &Vec<String>) -> i64 {
    let mut x0 = 0;
    let mut y0 = 0;

    let mut grid = Vec::new();

    let mut final_mask = 0 as i64;
    let mut mapping = HashMap::new();

    let mut cy = 0 as i64;
    for line in lines {
        let mut row = Vec::new();
        let mut cx = 0 as i64;
        for ch in line.chars() {
            row.push(ch);
            if ch == '@' {
                x0 = cx;
                y0 = cy;
            }
            if 'a' <= ch && ch <= 'z' {
                let index = (ch as u8 - 'a' as u8) as i64;
                let bit = (1 << index);
                assert!(final_mask & bit == 0);
                final_mask |= bit;
                mapping.insert(index, (cx, cy));
            }
            cx += 1;
        }
        grid.push(row);
        cy += 1
    }

    let mut dirs = Vec::new();
    dirs.push((-1, 0));
    dirs.push((1, 0));
    dirs.push((0, -1));
    dirs.push((0, 1));

    // let mut visited = HashMap::new();
    // let mut q = VecDeque::new();
    for iy in vec![y0 - 1, y0, y0 + 1] {
        for ix in vec![x0 - 1, x0, x0 + 1] {
            grid[iy as usize][ix as usize] = '#';
        }
    }

    for iy in vec![y0 - 1, y0 + 1] {
        for ix in vec![x0 - 1, x0 + 1] {
            grid[iy as usize][ix as usize] = '.';
        }
    }

    let mut sum = 0;
    for iy in vec![y0 - 1, y0 + 1] {
        for ix in vec![x0 - 1, x0 + 1] {
            println!("building graph from x={}, y={}", ix, iy);
            let value = bfs(ix, iy, &grid);
            sum += value;
        }
    }
    sum
}

fn main() {
    // let lines = read_input("day16/t2-t0.txt");

    // let lines = read_input("day18/t2.txt");
    let lines = read_input("day18/in.txt");

    // println!("part1 = {}", part1(&lines));
    // println!("part2 = {}", part2(&lines));
    println!("part2 = {}", part2_dummy(&lines));
}

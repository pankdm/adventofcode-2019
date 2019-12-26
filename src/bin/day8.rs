use std::fs::File;
use std::io::{BufRead, BufReader};


extern crate adventofcode;
use adventofcode::*;


pub fn part1(lines: &Vec<String>) -> i64 {
    // println!("read: {}", line);
    let size = 25 * 6;

    let line = &lines[0];

    let mut layers = Vec::new();
    let mut count = 0;
    for i in 0..line.len() {
        if count == size {
            count = 0;
        }
        if count == 0 {
            layers.push(vec![0, 0, 0]);
        }
        let ch = line[i..i+1].chars().next().unwrap();
        let index = layers.len() - 1;
        let num = ch as u32 - '0' as u32;
        layers[index][num as usize] += 1;
        count += 1;
    }

    let mut best = -1;
    let mut best_index = 0 as usize;
    for i in 0..layers.len() {
        if best == -1 || layers[i][0] < best {
            best = layers[i][0];
            best_index = i;
        }
    }

    let mut ans = layers[best_index][1] * layers[best_index][2];
    ans
}


pub fn part2(lines: &Vec<String>) -> i64 {
    let width = 25;
    let tall = 6;

    let line = &lines[0];

    // println!("read: {}", line);
    let size = width * tall;

    let mut layers = Vec::new();
    let mut pics = Vec::new();
    let mut count = 0;
    for i in 0..line.len() {
        if count == size {
            count = 0;
        }
        if count == 0 {
            layers.push(vec![0, 0, 0]);
            pics.push(Vec::new());
        }
        let ch = line[i..i+1].chars().next().unwrap();
        let index = layers.len() - 1;
        let num = ch as u32 - '0' as u32;
        layers[index][num as usize] += 1;
        pics[index].push(num);
        count += 1;
    }

    let mut ans = vec!['?' as char; size];
    for pos in 0..size {
        for i in 0..pics.len() {
            if pics[i][pos] == 0 {
                ans[pos] = ' ';
                break;
            }
            if pics[i][pos] == 1 {
                ans[pos] = '#';
                break;
            }
        } 
    }

    let mut counter = 0;
    for y in 0..tall {
        for x in 0..width {
            print!("{}", ans[counter]);
            counter += 1;
        }
        println!("");
    }
    -1
}


mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_input("day8/in.txt");
        assert_eq!(part1(&lines), 1560);
    }
}

fn main() {
    let lines = read_input("day8/in.txt");

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}

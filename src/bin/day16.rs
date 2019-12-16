extern crate adventofcode;
use adventofcode::*;

use std::collections::HashMap;
use std::io::{self, Write};
use std::time::{Duration, Instant};

fn to_digits(line: &String) -> Vec<i32> {
    let mut input = Vec::new();
    for ch in line.chars() {
        let digit = ch as u8 - '0' as u8;
        input.push(digit as i32);
    }
    return input;
}

pub fn part1(lines: &Vec<String>, times: usize) -> i64 {
    let line = &lines[0];
    let mut input = to_digits(line);
    // println!("{:?}", input);
    println!("len = {}", input.len());

    for i in 0..times {
        input = fft(input);

        print!("iter = {}: ", i);
        for i in 0..8 {
            print!("{}", input[i]);
        }
        print!("\n");
        io::stdout().flush().unwrap();
    }
    return -1;
}

fn last_digit(v: i32) -> i32 {
    return v.abs() % 10;
}

// fn create_pattern(len: usize, times: usize) -> Vec<(i32, i32)> {
//     let mut pattern = Vec::new();
//     let digits = vec![0, 1, 0, -1];
//     for d in &digits {
//             pattern.push(*d as i32);
//             if pattern.len() > len {
//                 return pattern;
//             }
//         }
//     }
//     return pattern;
// }

fn sum_query(preproc: &Vec<i32>, start: usize, end: usize) -> i32 {
    return preproc[end] - preproc[start];
}

fn multiply(input: &Vec<i32>, preproc: &Vec<i32>, start: usize, times: usize) -> i32 {
    // let pattern = create_pattern(input.len(), times);
    let pattern = vec![1, 0, -1, 0];

    let mut p_pos = 0;
    let mut i_pos = start;

    let mut sum = 0;
    loop {
        if i_pos > input.len() {
            break;
        }
        let value = sum_query(preproc, i_pos, (i_pos + times).min(input.len()));
        sum += value * pattern[p_pos % pattern.len()];

        i_pos += times;
        p_pos += 1;
    }

    let x = last_digit(sum) as i32;
    // println!("  at pos {} -> sum = {}", times, x);
    return x;
}

fn fft(input: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::new();
    let mut preproc = Vec::new();

    let mut sum = 0;
    preproc.push(0);
    for i in 0..input.len() {
        sum += input[i];
        preproc.push(sum);
    }

    for i in 0..input.len() {
        res.push(multiply(&input, &preproc, i, i + 1));
    }
    return res;
}

fn get_slice(input: &Vec<i32>, start: usize, count: usize) -> String {
    let mut res = String::new();
    for i in start..start + count {
        let ch = (input[i] as u8 + '0' as u8) as char;
        res.push(ch);
    }
    return res;
}

pub fn part2(lines: &Vec<String>) -> String {
    let line = &lines[0];
    let mut init = to_digits(line);
    // println!("{:?}", input);
    println!("init len = {}", init.len());

    let mut input = Vec::new();
    for i in 0..10_000 {
        for j in 0..init.len() {
            input.push(init[j]);
        }
    }
    println!("input len = {}", input.len());

    let mut offset = 0;
    for i in 0..7 {
        offset *= 10;
        offset += input[i];
    }
    println!("using offset {}", offset);

    let mut sum_elapsed = 0;
    for i in 0..100 {
        let now = Instant::now();
        input = fft(input);
        let elapsed = now.elapsed().as_millis();
        sum_elapsed += elapsed;

        let slice = get_slice(&input, offset as usize, 8);
        print!("iter = {}: {}", i, slice);
        println!(", in {} ms, avg = {}", elapsed, sum_elapsed / (i + 1));
        // io::stdout().flush().unwrap();
    }
    return get_slice(&input, offset as usize, 8);
}

fn main() {
    // let lines = read_input("day16/t0.txt");
    // let lines = read_input("day16/t2-t0.txt");

    let lines = read_input("day16/in.txt");

    // println!("part1 = {}", part1(&lines, 100));
    println!("part2 = {}", part2(&lines));
}

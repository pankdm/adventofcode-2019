use std::fs::File;
use std::io::{BufRead, BufReader};


const MID: i64 = 10000;
const N: i64 = 2 * MID;
const N2: i64 = N*N;

fn index(x: i64, y: i64) -> usize {
    assert!(-MID < x && x < MID);
    assert!(-MID < y && y < MID);
    let index = (x + MID) * N + (y + MID);
    assert!(0 <= index && index < N2);
    return index as usize;
}

fn apply_grid(line_tmp: String, grid: &mut Vec<i8>, bit: i8) {
    let line = line_tmp.trim();
    let mut x = 0;
    let mut y = 0;
    // grid[index(x, y)] |= bit;
    let steps = line.split(",").collect::<Vec<&str>>();
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
        // println!("parsing {}", slice);
        let num = slice.parse::<i64>().unwrap();
        // println!("parsed {}", slice);
        for i in 0..num {
            x += dx;
            y += dy;
            grid[index(x, y)] |= bit;
        }
    }
}


fn main() {
    // let filename = "test2.txt";
    // let filename = "test0.txt";
    let filename = "in.txt";

    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line1 = String::new();
    let mut line2 = String::new();

    let mut grid = vec![0 as i8; N2 as usize];

    reader.read_line(&mut line1);
    reader.read_line(&mut line2);

    let mut best = -1;
    apply_grid(line1, &mut grid, 1);
    apply_grid(line2, &mut grid, 2);
    for x in (-MID + 1)..MID {
        for y in (-MID + 1)..MID {
            if x == 0 && y == 0 {
                continue;
            }
            if grid[index(x, y)] == 3 {
                let dist = x.abs() + y.abs();
                if best == -1 || dist < best {
                    best = dist;
                }
            }
        }
    }
    println!("best = {}", best);
}
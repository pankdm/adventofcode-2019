use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};


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
                    println!("{:?} was not found before", &key);
                } else {
                    println!("  {:?} is already there", &key);
                }
                unique.insert(key);
            }
        }
    }
    return count;
}


fn main() {
    let filename = "in.txt";
    // let filename = "t0.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);


    let mut grid = Vec::new();
    for line in reader.lines() {
        grid.push(line.unwrap().trim().to_string());
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

    // let mut best = count_asteroid(&grid, 3, 4);

    println!("best = {}", best);

}
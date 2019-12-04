use std::fs::File;
use std::io::{BufRead, BufReader};


fn total_fuel(mass: i64) -> i64 {
    let mut sum:i64 = 0;
    let mut x = mass;
    while x >= 0 {
        let fuel = x / 3 - 2;
        if fuel >= 0 {
            sum += fuel;
        }
        x = fuel;
    }
    return sum;
}

fn main() {
    let filename = "in.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut sum:i64 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let mass = line.parse::<i64>().unwrap();
        sum += total_fuel(mass);
        // println!("read: {}", number);
    }
    println!("sum = {}", sum);
}
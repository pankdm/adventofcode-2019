use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
    let filename = "in.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut sum:i64 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let number = line.parse::<i64>().unwrap();
        sum += number / 3 - 2;
        // println!("read: {}", number);
    }
    println!("sum = {}", sum);
}
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::{self, Write};

fn main() {
    // let filename = "test.txt";
    let filename = "in.txt";

    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line);

    let mut ans = -1;
    println!("ans = {}", ans);
}
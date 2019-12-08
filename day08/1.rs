use std::fs::File;
use std::io::{BufRead, BufReader};


// fn process_ops(ops: Vec<i64>) -> i64 {

// }

fn main() {
    let filename = "in.txt";
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line);

    // println!("read: {}", line);
    let size = 25 * 6;

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
    println!("ans = {}", ans);
}
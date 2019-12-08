use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::{self, Write};


// fn process_ops(ops: Vec<i64>) -> i64 {

// }

fn main() {
    // let filename = "test.txt";
    let filename = "in.txt";
    let width = 25;
    let tall = 6;


    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line);

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
        print!("\n");
        io::stdout().flush().unwrap();
    }
}
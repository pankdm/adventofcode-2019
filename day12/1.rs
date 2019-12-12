use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};


const p: i64 = 1000003;

fn main() {
    // let filename = "in.txt";
    // let filename = "t1.txt";
    // let STEPS = 1000;

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut pos = Vec::new();
    let mut vel = Vec::new();


    let mut count = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let len = line.len();
        let kvs = line[1..len - 1].split(",").collect::<Vec<&str>>();

        let mut cur_pos = Vec::new();
        let mut cur_vel = Vec::new();
        for kv in kvs.clone() {
            let parts = kv.split("=").collect::<Vec<&str>>();
            let v = parts[1].parse::<i64>().unwrap();
            cur_pos.push(v);
            cur_vel.push(0);
        }
        pos.push(cur_pos);
        vel.push(cur_vel);

        count += 1;
    }

    let mut before = HashSet::new();

    let mut pows = Vec::new();
    pows.push(1 as u64);
    for i in 0..(pos.len() * 2 * 3 + 1) {
        let mut next = pows[i] * p;
        pows.push(next as u64);
    } 

    let mut step = 0;
    loop {
        for i in 0..pos.len() {
            for j in (i + 1)..pos.len() {
                for c in 0..3 {
                    if pos[i][c] > pos[j][c] {
                        vel[i][c] -= 1;
                        vel[j][c] += 1;
                    } else if pos[i][c] < pos[j][c] {
                        vel[i][c] += 1;
                        vel[j][c] -= 1;
                    }
                }
            }
        }

        for i in 0..pos.len() {
            for c in 0..3 {
                pos[i][c] += vel[i][c];
            }
        }


        let mut hash = 0 as u64;
        let mut cnt = 0;
        for i in 0..pos.len() {
            for c in 0..3 {
                hash += pows[cnt] (* vel[i][c];
                cnt += 1;
                hash += pows[cnt] * pos[i][c];
                cnt += 1;
            }
        }
        if before.contains(hash) {
            println!("steps = {}", step);
            break;
        }
        before.insert(hash);

        step += 1;

        // println!("");
        // println!("after {} steps:", step + 1);
        // for i in 0..pos.len() {
        //     println!("pos = {:?}, vel = {:?}",  &pos[i], &vel[i]);
        // }
    }

    // let mut ans = 0;
    // for i in 0..pos.len() {
    //     let mut kin = 0;
    //     let mut pot = 0;
    //     for c in 0..3 {
    //         kin += pos[i][c].abs();
    //         pot += vel[i][c].abs();
    //     }
    //     ans += kin * pot;
    // }

    println!("ans = {}", ans);
}
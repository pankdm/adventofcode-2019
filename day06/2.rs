use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;


fn go(rev: &mut HashMap<String, String>, now: String, count: i64, res: &mut HashMap<String, i64>) {
    let next = rev.entry(now.clone()).or_default().clone();
    res.entry(now.clone()).or_insert(count);
    if next != "" {
        go(rev, next.to_string(), count + 1, res);        
    }
}


fn main() {
    let filename = "in.txt";
    // let filename = "test2.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut map: HashMap<String, Vec<String> > = HashMap::new();
    let mut rev: HashMap<String, String> = HashMap::new();

    for line in reader.lines() {
        let line1 = line.unwrap();
        let l = line1.split(")").collect::<Vec<&str>>();
        let from = l[0].to_string();
        let to = l[1].to_string();

        let from1 = from.clone();
        let to1 = to.clone();
        map.entry(from).or_insert(Vec::new()).push(to);
        rev.entry(to1).or_insert(from1);
    }
    let mut start = "";

    let map_iter = map.clone();
    {
        for (from, to) in map_iter.iter() {
            if !rev.contains_key(from) {
                start = from;
                break;
            }
        }
        assert!(start != "");
    }

    println!("start = {}", start);

    let mut you: HashMap<String, i64> = HashMap::new();
    let mut santa: HashMap<String, i64> = HashMap::new();
    go(&mut rev, "YOU".to_string(), 0, &mut you);
    go(&mut rev, "SAN".to_string(), 0, &mut santa);

    let mut ans = -1;

    for (k, v1) in you.iter() {
        match santa.get(k) {
            Some(&v2) => {
                let best = v1 + v2;
                if ans == -1 || best < ans {
                    ans = best
                }
            }
            _ => {}
        }
    }


    println!("ans = {}", ans - 2);
}
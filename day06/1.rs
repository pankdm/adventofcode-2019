use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;


fn go(g: &mut HashMap<String, Vec<String>>, v: String, count: i64, ans: &mut i64) {
    *ans += count;
    let nexts = g.entry(v).or_default().clone();
    for next in nexts {
        go(g, next.to_string(), count + 1, ans);
    }
}


fn main() {
    let filename = "in.txt";
    // let filename = "test.txt";

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

    let mut ans = 0;
    go(&mut map, start.to_string(), 0, &mut ans);

    println!("ans = {}", ans);
}
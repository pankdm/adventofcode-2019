use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};



struct Elem {
    cnt: i64,
    type_: String,
}

struct Reaction {
    output: Elem,
    inputs: Vec<Elem>,
}


// fn go(now: String, cnt: i64, rs: &HashMap<String, Reaction>) -> i64 {
//     if now == "ORE" {
//         return cnt;
//     }

//     let reaction = rs.get(&now).unwrap();
//     let mut sum = 0;
//     for input in &reaction.inputs {
//         let value = go(input.type_.clone(), input.cnt, rs);
//         sum += value;
//     }
//     let getting = reaction.output.cnt;
//     let times = (cnt + getting - 1) / getting;
//     let res = times * sum;
//     println!("To get {} of {} need {} ORE || times = {}, getting = {}", cnt, now, res, times, getting);
//     return res;
// }

fn iterate(desired_: &HashMap<String, i64>, rs: &HashMap<String, Reaction>, graph: &HashMap<String, HashSet<String>>) -> i64 {
    let mut desired = desired_.clone();
    let ore_key = "ORE".to_string();
    loop {
        let mut best_key = "".to_string();
        for (key, value) in &desired {
            if best_key == "" {
                best_key = key.to_string();
            }
            match graph.get(key) {
                Some(ref from) => {
                    if from.contains(&best_key) {
                        best_key = key.to_string();
                    }
                }
                _ => {}
            }
        }
        println!("found best key = {}", &best_key);
        if best_key == "ORE" {
            if desired.len() != 1 {
                println!("   WARNING: {:?}", &desired);
            }
            return desired[&best_key];
        }

        let reaction = rs.get(&best_key).unwrap().clone();
        let need = desired[&best_key];
        let getting = reaction.output.cnt;
        let times = (need + getting - 1) / getting;

        println!("unwrapping {} of {} elem", need, &best_key);


        desired.remove(&best_key);
        for input in &reaction.inputs {
            let extra = times * input.cnt;
            let current = *desired.entry(input.type_.clone()).or_default();
            let new = current + extra;
            desired.insert(input.type_.to_string(), new);
            println!("   added {} to {} -> became {}", extra, &input.type_, new);
        }
    }
}

fn go_graph(now: String, rs: &HashMap<String, Reaction>, graph: &mut HashMap<String, HashSet<String>>) -> HashSet<String> {
    let mut res = HashSet::new();
    match graph.get(&now) {
        Some(ref rr) => {
            return (*rr).clone();
        }
        _ => {}
    }

    match rs.get(&now) {
        Some(ref reaction) => {
            for input in &reaction.inputs {
                let res2 = go_graph(input.type_.clone(), rs, graph);
                for r in &res2 {
                    res.insert(r.clone());
                }
                res.insert(input.type_.to_string());
            }
        }
        _ => {}
    }
    graph.insert(now, res.clone());
    return res;
}


fn main() {
    let filename = "in.txt";
    // let filename = "t1.txt";
    // let STEPS = 1000;

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut rs = HashMap::new();

    for line in reader.lines() {
        let line_ = line.unwrap();
        let parts = line_.split(" => ").collect::<Vec<&str>>();
        let _inputs = parts[0].split(", ").collect::<Vec<&str>>();

        let mut inputs = Vec::new();
        for input_ in _inputs {
            let input = input_.split(" ").collect::<Vec<&str>>();
            let cnt = input[0].parse::<i64>().unwrap();
            let type_ = input[1].to_string();
            let e_in = Elem {
                cnt, type_
            };
            inputs.push(e_in);
        }

        let outputs = parts[1].split(" ").collect::<Vec<&str>>();
        let cnt = outputs[0].parse::<i64>().unwrap();
        let type_ = outputs[1].to_string();
        let output = Elem{cnt, type_};
        let r = Reaction{output, inputs};
        rs.insert(r.output.type_.clone(), r);
    }

    let start = "FUEL".to_string();
    let mut graph = HashMap::new();
    go_graph(start.clone(), &rs, &mut graph);
    for (k, v) in &graph {
        println!("|| at key {} --> {:?}", k, v);
    }
    println!("  ");


    let mut desired = HashMap::new();
    desired.insert(start.clone(), 1);
    let ans = iterate(&desired, &rs, &graph);

    // let ans = go(start, 1, &rs);
    println!("ans = {}", ans);
}
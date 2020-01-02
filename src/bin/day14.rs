use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

extern crate adventofcode;
use adventofcode::*;

struct Elem {
    cnt: i64,
    type_: String,
}

struct Reaction {
    output: Elem,
    inputs: Vec<Elem>,
}

type ReactionMap = HashMap<String, Reaction>;
type DepGraph = HashMap<String, HashSet<String>>;

fn search(rs: &ReactionMap, graph: &DepGraph) -> i64 {
    let TRILLION = 1000000000000;
    let start = "FUEL".to_string();
    let mut lo = 1;
    let mut hi = TRILLION / 1000;
    while lo + 1 < hi {
        let mid = (lo + hi) / 2;

        let mut desired = HashMap::new();
        desired.insert(start.clone(), mid);

        let res = iterate(&desired, &rs, &graph);
        if res <= TRILLION {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    return lo;
}

fn iterate(desired_: &HashMap<String, i64>, rs: &ReactionMap, graph: &DepGraph) -> i64 {
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

fn go_graph(now: String, rs: &ReactionMap, graph: &mut DepGraph) -> HashSet<String> {
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

pub fn part1(lines: &Vec<String>) -> i64 {
    let rs = parse_reactions(lines);

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
    ans
}

fn parse_reactions(lines: &Vec<String>) -> ReactionMap {
    let mut rs = HashMap::new();

    for line in lines {
        let parts = split_string(&line, " => ");
        let inputs_ = split_string(&parts[0], ", ");

        let mut inputs = Vec::new();
        for input_ in inputs_ {
            let input = split_string(&input_, " ");
            let cnt = parse_i64(&input[0]);
            let type_ = input[1].to_string();
            let e_in = Elem { cnt, type_ };
            inputs.push(e_in);
        }

        let outputs = split_string(&parts[1], " ");
        let cnt = parse_i64(&outputs[0]);
        let type_ = outputs[1].to_string();
        let output = Elem { cnt, type_ };
        let r = Reaction { output, inputs };
        rs.insert(r.output.type_.clone(), r);
    }
    rs
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let rs = parse_reactions(lines);

    let mut graph = HashMap::new();
    let start = "FUEL".to_string();
    go_graph(start.clone(), &rs, &mut graph);
    for (k, v) in &graph {
        println!("|| at key {} --> {:?}", k, v);
    }
    println!("  ");

    let ans = search(&rs, &graph);

    // let ans = go(start, 1, &rs);
    println!("ans = {}", ans);
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_input("day14/in.txt");
        assert_eq!(part1(&lines), 1967319);
    }

    #[test]
    fn test_part2() {
        let lines = read_input("day14/in.txt");
        assert_eq!(part2(&lines), 1122036);
    }
}

fn main() {
    let lines = read_input("day14/in.txt");

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}

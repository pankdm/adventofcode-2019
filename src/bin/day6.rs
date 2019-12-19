extern crate adventofcode;
use adventofcode::*;

use std::collections::HashMap;

type Graph = HashMap<String, Vec<String>>;

fn go(graph: &Graph, v: String, count: i64, ans: &mut i64) {
    *ans += count;
    match graph.get(&v) {
        Some(ref nexts) => {
            for next in *nexts {
                go(graph, next.to_string(), count + 1, ans);
            }
        }
        _ => {}
    }
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut graph = HashMap::new();
    let mut back = HashMap::new();

    for line in lines {
        let l = split_string(line, ")");
        let from = l[0].to_string();
        let to = l[1].to_string();

        graph
            .entry(from.clone())
            .or_insert(Vec::new())
            .push(to.clone());
        back.insert(to, from);
    }
    let mut start = "".to_string();

    for (from, _to) in &graph {
        if !back.contains_key(from) {
            start = from.clone();
            break;
        }
    }
    assert!(start != "");

    println!("start = {}", start);

    let mut ans = 0;
    go(&graph, start, 0, &mut ans);

    return ans;
}

fn go2(back: &HashMap<String, String>, now: &String, count: i64, res: &mut HashMap<String, i64>) {
    res.entry(now.clone()).or_insert(count);

    match back.get(now) {
        Some(ref next) => {
            go2(back, &next, count + 1, res);
        }
        _ => {}
    }
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut graph = HashMap::new();
    let mut back = HashMap::new();

    for line in lines {
        let l = split_string(line, ")");
        let from = l[0].to_string();
        let to = l[1].to_string();

        graph
            .entry(from.clone())
            .or_insert(Vec::new())
            .push(to.clone());
        back.insert(to, from);
    }

    let mut you: HashMap<String, i64> = HashMap::new();
    let mut santa: HashMap<String, i64> = HashMap::new();
    go2(&back, &"YOU".to_string(), 0, &mut you);
    go2(&back, &"SAN".to_string(), 0, &mut santa);

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
    ans - 2
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_input("day6/in.txt");
        assert_eq!(part1(&lines), 119831);
    }

    #[test]
    fn test_part1_example1() {
        let input = r#"
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
"#;
        assert_eq!(part1(&to_lines(input)), 42);
    }

    #[test]
    fn test_part2_example1() {
        let input = r#"
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN
"#;
        assert_eq!(part2(&to_lines(input)), 4);
    }

    #[test]
    fn test_part2() {
        let lines = read_input("day6/in.txt");
        assert_eq!(part2(&lines), 322);
    }
}

fn main() {
    let lines = read_input("day6/in.txt");

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}

extern crate adventofcode;
use adventofcode::*;

use std::collections::BTreeSet;
use std::collections::{HashMap, HashSet, VecDeque};

// pub fn part1(lines: &Vec<String>) -> i128 {
//     -1
// }


#[derive(Copy, Clone)]
enum Command {
    New,
    Cut(i128),
    Increment(i128),
}

type Deck = Vec<i128>;


#[derive(Copy, Clone)]
struct FastDeck {
    k: i128,
    b: i128,
    modulo: i128,
}


fn mod_op(a: i128, modulo: i128) -> i128 {
    (a % modulo + modulo) % modulo
}


fn reduce(f: &mut FastDeck) {
    f.k = mod_op(f.k, f.modulo);
    f.b = mod_op(f.b, f.modulo);
}

fn standard_deck(size: usize) -> Deck {
    let mut d = Deck::new();
    for i in 0..size {
        d.push(i as i128);
    }
    d
}

fn apply_new(d: &Deck) -> Deck {
    let mut res = d.clone();
    res.reverse();
    res
}

fn apply_cut(d: &Deck, _value: i128) -> Deck {
    let mut value = _value;

    assert!((value.abs() as usize) < d.len());
    if value < 0 {
        value = d.len() as i128 + value;
    }
    let mut res = d.clone();
    res.rotate_left(value as usize);
    res
}

fn apply_increment(d: &Deck, value: i128) -> Deck {
    let mut cnt = 0 as usize;
    let mut res = vec![-1; d.len()];
    for i in 0..d.len() {
        res[cnt] = d[i];
        cnt += value as usize;
        if cnt >= d.len() {
            cnt -= d.len();
        }
    }

    for &v in &res {
        assert_ne!(v, -1);
    }

    res
}

fn apply_commands(lines: &Vec<String>, size: usize) -> Deck {
    let cmds = parse_commands(lines);
    let mut d = standard_deck(size);

    for &cmd in &cmds {
        match cmd {
            Command::New => {d = apply_new(&d);},
            Command::Cut(value) => {d = apply_cut(&d, value);},
            Command::Increment(value) => {d = apply_increment(&d, value);}
        }
    }
    d
}


fn standard_deck2(size: usize) -> FastDeck {
    FastDeck {
        k: 1,
        b: 0,
        modulo: size as i128,
    }
}

fn apply_new2(d: &FastDeck) -> FastDeck {
    let mut f = FastDeck{
        k: -d.k,
        b: -d.b - 1,
        modulo: d.modulo
    };
    reduce(&mut f);
    f
}

fn apply_cut2(d: &FastDeck, value: i128) -> FastDeck {
    let mut f = FastDeck{
        k: d.k,
        b: d.b - value,
        modulo: d.modulo,
    };
    reduce(&mut f);
    f
}

fn apply_increment2(d: &FastDeck, value: i128) -> FastDeck {
    let mut f = FastDeck{
        k: value * d.k,
        b: value * d.b,
        modulo: d.modulo,
    };
    reduce(&mut f);
    f
}

fn apply_commands2(lines: &Vec<String>, size: usize) -> FastDeck {
    let cmds = parse_commands(lines);
    let mut d = standard_deck2(size);

    for &cmd in &cmds {
        match cmd {
            Command::New => {d = apply_new2(&d);},
            Command::Cut(value) => {d = apply_cut2(&d, value);},
            Command::Increment(value) => {d = apply_increment2(&d, value);}
        }
    }
    d
}

fn parse_commands(lines: &Vec<String>) -> Vec<Command> {
    let mut cmds = Vec::new();

    for l in lines {
        let words = split_string(l, " ");
        if words[0] == "cut" {
            let value = parse_i64(&words[1]) as i128;
            cmds.push(Command::Cut(value));
            continue;
        }
        if words[2] == "increment" {
            let value = parse_i64(&words[3]) as i128;
            cmds.push(Command::Increment(value));
            continue;
        }
        if words[2] == "new" {
            cmds.push(Command::New);
            continue;
        }
        unreachable!();
    }
    cmds
}

pub fn part1(lines: &Vec<String>) -> i128 {
    let d = apply_commands(lines, 10007);
    let f = apply_commands2(lines, 10007);


    for i in 0..d.len() {
        let value = reverse(&f, i as i128);
        assert_eq!(value, d[i]);
    }


    for i in 0..d.len() {
        if d[i] == 2019 {
            return i as i128;
        }
    }


    unreachable!();
}

pub fn part2(lines: &Vec<String>) -> i128 {
    let p = 119315717514047;
    let mut f = apply_commands2(lines, p);
    f = f_to_pow(&f, 101741582076661);
    reverse(&f, 2020)
}



fn as_deck(s: &str) -> Deck {
    let mut d = Deck::new();
    for part in split_string(&s.to_string(), " ") {
        d.push(parse_i64(&part) as i128);
    }
    d
}


fn pow_mod(a: i128, n: i128, p: i128) -> i128 {
    if n == 0 {
        return 1;
    }
    if n == 1 {
        return a;
    }
    if n % 2 == 1 {
        mod_op(a * pow_mod(a, n - 1, p), p)
    } else {
        pow_mod(mod_op(a * a, p), n / 2, p)
    }
}


fn reverse(f: &FastDeck, value: i128) -> i128 {
    let p = f.modulo;
    let rev_k = pow_mod(f.k, p - 2, p);
    mod_op((value - f.b) * rev_k, p)
    // k * x + b = v
    // x = (v - b) / k
    // reverse(k) mod p == k**(p - 2) mod p
}

fn f_to_pow(f: &FastDeck, n: i128) -> FastDeck {
    let k = f.k;
    let p = f.modulo;
    let rev_k_1 = pow_mod(k - 1, p - 2, p);
    let k_pow_n = pow_mod(k, n, p);
    let mut f_new = FastDeck{
        k: k_pow_n,
        b: f.b * mod_op((k_pow_n - 1) * rev_k_1, p),
        modulo: p,
    };
    reduce(&mut f_new);
    f_new
}


fn to_deck(f: &FastDeck) -> Deck {
    let mut d = vec![-1 ; f.modulo as usize];
    for x in 0..f.modulo {
        let pos = mod_op(f.k * x + f.b, f.modulo) as usize;
        d[pos] = x as i128;
    }

    for &v in &d {
        assert_ne!(v, -1);
    }
    d
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment() {
        let mut d = standard_deck(10);
        d = apply_increment(&d, 3);
        assert_eq!(d, as_deck("0 7 4 1 8 5 2 9 6 3"));
    }


    #[test]
    fn test_cut() {
        let mut d = standard_deck(10);
        d = apply_cut(&d, 3);
        assert_eq!(d, as_deck("3 4 5 6 7 8 9 0 1 2"));
    }

    #[test]
    fn test_cut_negative() {
        let mut d = standard_deck(10);
        d = apply_cut(&d, -4);
        assert_eq!(d, as_deck("6 7 8 9 0 1 2 3 4 5"));
    }

    #[test]
    fn test_new() {
        let mut d = standard_deck(10);
        d = apply_new(&d);
        assert_eq!(d, as_deck("9 8 7 6 5 4 3 2 1 0"));
    }

    #[test]
    fn test_example1() {
        let input = r#"
deal with increment 7
deal into new stack
deal into new stack
"#.trim();
        let d = apply_commands(&to_lines(input), 10);
        assert_eq!(d, as_deck("0 3 6 9 2 5 8 1 4 7"));
    }

    #[test]
    fn test_example2() {
        let input = r#"
cut 6
deal with increment 7
deal into new stack
"#.trim();
        let d = apply_commands(&to_lines(input), 10);
        assert_eq!(d, as_deck("3 0 7 4 1 8 5 2 9 6"));
    }



    #[test]
    fn test_example3() {
        let input = r#"
deal with increment 7
deal with increment 9
cut -2
"#.trim();
        let d = apply_commands(&to_lines(input), 10);
        assert_eq!(d, as_deck("6 3 0 7 4 1 8 5 2 9"));
    }

    #[test]
    fn test_example4() {
        let input = r#"
deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1
"#.trim();
        let d = apply_commands(&to_lines(input), 10);
        assert_eq!(d, as_deck("9 2 5 8 1 4 7 0 3 6"));
    }


    #[test]
    fn test_fast_increment() {
        let mut d = standard_deck2(10);
        d = apply_increment2(&d, 3);
        assert_eq!(to_deck(&d), as_deck("0 7 4 1 8 5 2 9 6 3"));
    }


    #[test]
    fn test_fast_cut() {
        let mut d = standard_deck2(10);
        d = apply_cut2(&d, 3);
        assert_eq!(to_deck(&d), as_deck("3 4 5 6 7 8 9 0 1 2"));
    }

    #[test]
    fn test_fast_cut_negative() {
        let mut d = standard_deck2(10);
        d = apply_cut2(&d, -4);
        assert_eq!(to_deck(&d), as_deck("6 7 8 9 0 1 2 3 4 5"));
    }

    #[test]
    fn test_fast_new() {
        let mut d = standard_deck2(10);
        d = apply_new2(&d);
        assert_eq!(to_deck(&d), as_deck("9 8 7 6 5 4 3 2 1 0"));
    }

    #[test]
    fn test_fast_example1() {
        let input = r#"
deal with increment 7
deal into new stack
deal into new stack
"#.trim();
        let d = apply_commands2(&to_lines(input), 10);
        assert_eq!(to_deck(&d), as_deck("0 3 6 9 2 5 8 1 4 7"));
    }

    #[test]
    fn test_fast_example2() {
        let input = r#"
cut 6
deal with increment 7
deal into new stack
"#.trim();
        let d = apply_commands2(&to_lines(input), 10);
        assert_eq!(to_deck(&d), as_deck("3 0 7 4 1 8 5 2 9 6"));
    }



    #[test]
    fn test_fast_example3() {
        let input = r#"
deal with increment 7
deal with increment 9
cut -2
"#.trim();
        let d = apply_commands2(&to_lines(input), 10);
        assert_eq!(to_deck(&d), as_deck("6 3 0 7 4 1 8 5 2 9"));
    }

    #[test]
    fn test_fast_example4() {
        let input = r#"
deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1
"#.trim();
        let d = apply_commands2(&to_lines(input), 10);
        assert_eq!(to_deck(&d), as_deck("9 2 5 8 1 4 7 0 3 6"));
    }

    #[test]
    fn test_part1() {
        let lines = read_input("day22/in.txt");
        assert_eq!(part1(&lines), 3324);
    }

    #[test]
    fn test_part2() {
        let lines = read_input("day22/in.txt");
        assert_eq!(part2(&lines), 74132511136410);
    }
}

fn main() {
    let lines = read_input("day22/in.txt");

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}

extern crate mylib;

use mylib::*;

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut sum:i64 = 0;
    for line in lines {
        let number = parse_i64(line);
        sum += number / 3 - 2;
    }
    return sum;
}

fn total_fuel(mass: i64) -> i64 {
    let mut sum:i64 = 0;
    let mut x = mass;
    while x >= 0 {
        let fuel = x / 3 - 2;
        if fuel >= 0 {
            sum += fuel;
        }
        x = fuel;
    }
    return sum;
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut sum:i64 = 0;
    for line in lines {
        let number = parse_i64(line);
        sum += total_fuel(number);
    }
    return sum;    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_input("day1/in.txt");
        assert_eq!(part1(&lines), 3229279);
    }

    #[test]
    fn test_part2() {
        let lines = read_input("day1/in.txt");
        assert_eq!(part2(&lines), 4841054);
    }

    #[test]
    fn test_14() {
        assert_eq!(total_fuel(14), 2);
    }

    #[test]
    fn test_1969() {
        assert_eq!(total_fuel(1969), 966);
    }

    #[test]
    fn test_big() {
        assert_eq!(total_fuel(100756), 50346);
    }
}

fn main() {
    let lines = read_input("day1/in.txt");

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}
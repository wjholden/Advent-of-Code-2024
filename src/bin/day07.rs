use std::{fs};

struct Calibration {
    res: u64,
    val: Vec<u64>
}

fn main() {
    let puzzle = fs::read_to_string("puzzles/day07.txt").unwrap();
    println!("Part 1: {}", part1(&puzzle));
    println!("Part 2: {}", part2(&puzzle));
}

fn part1(input: &str) -> u64 {
    let calibrations = parse(input);
    calibrations.iter().filter(|c| {
        is_solvable1(0, &c.val, c.res)
    }).map(|c| c.res).sum()
}

fn part2(input: &str) -> u64 {
    let calibrations = parse(input);
    calibrations.iter().filter(|c| {
        is_solvable2(0, &c.val, c.res)
    }).map(|c| c.res).sum()
}

fn is_solvable1(left: u64, right: &[u64], target: u64) -> bool {
    if right.len() == 0 {
        left == target
    } else {
        let current = right[0];
        is_solvable1(left + current, &right[1..], target) ||
        is_solvable1(left.max(1) * current, &right[1..], target)
    }
}

fn is_solvable2(left: u64, right: &[u64], target: u64) -> bool {
    if right.len() == 0 {
        left == target
    } else {
        let current = right[0];
        let digits = 1 + (current as f64).log10().floor() as u32;
        is_solvable2(left + current, &right[1..], target) ||
        is_solvable2(left.max(1) * current, &right[1..], target) ||
        is_solvable2(left * 10u64.pow(digits) + current, &right[1..], target)
    }
}

fn parse(input: &str) -> Vec<Calibration> {
    input.trim().split('\n').map(|line| {
        let line: Vec<&str> = line.split(":").collect();
        assert_eq!(line.len(), 2);
        let res = line[0].parse().unwrap();
        let val = line[1].split_whitespace().into_iter().map(|s| s.parse().unwrap()).collect();
        Calibration {
            res,
            val,
        }
    }).collect()
}

#[cfg(test)]
mod day07 {
    use std::assert_eq;

    use super::*;

    const SAMPLE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn test1() {
        assert_eq!(part1(SAMPLE), 3749)
    }
 
    #[test]
    fn test2() {
        assert_eq!(part2(SAMPLE), 11387)
    }   

    #[test]
    fn test3() {
        assert_eq!(part2("192: 17 8 14"), 192);
    }
}
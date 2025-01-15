use std::{fs, num::ParseIntError, str::FromStr};
use rayon::prelude::*;

struct Calibration {
    res: u64,
    val: Vec<u64>
}

impl FromStr for Calibration {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line: Vec<&str> = s.split(":").collect();
        assert_eq!(line.len(), 2);
        let res = line[0].parse()?;
        let val = line[1].split_whitespace().map(|s| s.parse().unwrap()).collect();
        Ok(Self {
            res,
            val,
        })
    }
}

// Why is this solution so much faster than mine? 
// https://www.reddit.com/r/adventofcode/comments/1h8l3z5/comment/m0wq0ta/
fn main() {
    //let time = std::time::Instant::now();
    let puzzle = fs::read_to_string("puzzles/day07.txt").unwrap();
    let calibrations = parse(&puzzle);
    println!("Part 1: {}", part1(&calibrations));
    println!("Part 2: {}", part2(&calibrations));
    //println!("Time: {:?}", time.elapsed());
}

fn part1(calibrations: &[Calibration]) -> u64 {
    calibrations.par_iter().filter(|c| {
        is_solvable1(c.val[0], &c.val[1..], c.res)
    }).map(|c| c.res).sum()
}

fn part2(calibrations: &[Calibration]) -> u64 {
    calibrations.par_iter().filter(|c| {
        is_solvable2(c.val[0], &c.val[1..], c.res)
    }).map(|c| c.res).sum()
}

fn is_solvable1(left: u64, right: &[u64], target: u64) -> bool {
    if right.is_empty() {
        left == target
    } else if left > target {
        false
    } else {
        let current = right[0];
        is_solvable1(left + current, &right[1..], target) ||
        is_solvable1(left * current, &right[1..], target)
    }
}

fn is_solvable2(left: u64, right: &[u64], target: u64) -> bool {
    if right.is_empty() {
        left == target
    } else if left > target {
        false // our operators only increase the value, so stop early if we've already overflowed  
    } else {
        let current = right[0];
        // https://www.reddit.com/r/adventofcode/comments/1h8l3z5/comment/m0vp3p7/
        //let digits = 1 + (current as f64).log10().floor() as u32;
        let digits = 1 + current.ilog10(); // WOW, this is a lot faster. 
        is_solvable2(left + current, &right[1..], target) ||
        is_solvable2(left * current, &right[1..], target) ||
        is_solvable2(left * 10u64.pow(digits) + current, &right[1..], target)
    }
}

fn parse(input: &str) -> Vec<Calibration> {
    input.trim().split('\n').map(|line| Calibration::from_str(line).unwrap()).collect()
}

#[cfg(test)]
mod day07 {
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
        assert_eq!(part1(&parse(SAMPLE)), 3749)
    }
 
    #[test]
    fn test2() {
        assert_eq!(part2(&parse(SAMPLE)), 11387)
    }   

    #[test]
    fn test3() {
        assert_eq!(part2(&parse("192: 17 8 14")), 192);
    }
}
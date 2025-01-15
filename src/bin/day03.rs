use std::fs;
use regex::Regex;

fn main() {
    let puzzle = fs::read_to_string("puzzles/day03.txt").unwrap();
    println!("Part 1: {}", part1(&puzzle));
    println!("Part 2: {}", part2(&puzzle));
}

fn part1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((?P<x>\d+),(?P<y>\d+)\)").unwrap();
    re.captures_iter(input).map(|cap| {
        let x = &cap["x"];
        let y = &cap["y"];
        (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
    }).map(|(x,y)| x * y).sum()
}

fn part2(input: &str) -> i32 {
    let re = Regex::new(r"(?P<state>don't\(\)|do\(\))|mul\((?P<x>\d+),(?P<y>\d+)\)").unwrap();
    let mut mul_on = true;
    re.captures_iter(input).filter_map(|cap| {
        let group = (cap.name("state"), cap.name("x"), cap.name("y"));
        match group {
            (None, Some(x), Some(y)) if mul_on => 
                Some((x.as_str().parse::<i32>().unwrap(), y.as_str().parse::<i32>().unwrap())),
            (Some(state), ..) => {
                mul_on = state.as_str() == "do()";
                None
            },
            _ => None, // multiplication must be turned off. Ignore this mul(x,y) instruction.
        }
    }).map(|(x,y)| x * y).sum()
}

#[cfg(test)]
mod day03 {
    use super::*;

    const SAMPLE1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const SAMPLE2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test1() {
        assert_eq!(part1(SAMPLE1), 161)
    }
 
    #[test]
    fn test2() {
        assert_eq!(part2(SAMPLE2), 48)
    }   
}
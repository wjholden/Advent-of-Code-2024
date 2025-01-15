use std::cmp::Ordering;
use std::fs;
use std::collections::HashMap;

use regex::Regex;

fn main() {
    let puzzle = fs::read_to_string("puzzles/day05.txt").unwrap();
    let (p1, p2) = solve(&puzzle);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn parse(input: &str) -> (HashMap<(u16,u16), Ordering>,Vec<&str>) {
    let s: Vec<&str> = input.trim().split("\n\n").collect();

    let re = Regex::new(r"(?P<x>\d{2})\|(?P<y>\d{2})").unwrap();
    let mut rules = HashMap::new();
    re.captures_iter(s[0]).for_each(|cap| {
        let x = &cap["x"];
        let y = &cap["y"];
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();
        rules.insert((x,y), Ordering::Less);
        rules.insert((y,x), Ordering::Greater);
    });

    let updates: Vec<&str> = s[1].split("\n").collect();

    (rules, updates)
}

fn solve(input: &str) -> (u16,u16) {
    let (rules, updates) = parse(input);

    let mut middle_page_sum1 = 0;
    let mut middle_page_sum2 = 0;
    for update in updates {
        let mut pages: Vec<u16> = update.split(",").map(|e| e.parse().unwrap()).collect();
        let comparator = |&a,&b| *rules.get(&(a,b)).unwrap() == Ordering::Less;

        if pages.is_sorted_by(comparator) {
            let middle = pages[pages.len()/2];
            middle_page_sum1 += middle;
        } else {
            pages.sort_by(|&a,&b| *rules.get(&(a,b)).unwrap());
            let middle = pages[pages.len()/2];
            middle_page_sum2 += middle;
        }
        
    }
    (middle_page_sum1, middle_page_sum2)
}

#[cfg(test)]
mod day05 {
    use super::*;

    const SAMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test1() {
        assert_eq!(solve(SAMPLE).0, 143)
    }
 
    #[test]
    fn test2() {
        assert_eq!(solve(SAMPLE).1, 123)
    }   
}
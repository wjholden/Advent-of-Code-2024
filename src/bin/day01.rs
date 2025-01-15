use std::{collections::HashMap, fs, iter::zip};

fn main() {
    let puzzle = fs::read_to_string("puzzles/day01.txt").unwrap();
    println!("Part 1: {}", part1(&puzzle));
    println!("Part 2: {}", part2(&puzzle));
}

fn parse(x: &str) -> (Vec<i32>, Vec<i32>) {
    let mut iterator = x.split_whitespace();
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    while let (Some(i), Some(j)) = (iterator.next(), iterator.next()) {
        list1.push(i.parse().unwrap());
        list2.push(j.parse().unwrap());
    }
    list1.sort();
    list2.sort();
    (list1, list2)
}

fn part1(x: &str) -> i32 {
    let (list1, list2) = parse(x);
    zip(list1, list2).map(|(i,j)| (i-j).abs()).sum()
}

fn tally(v: &[i32]) -> HashMap<i32, i32> {
    let mut h = HashMap::new();
    for i in v.iter() {
        *h.entry(*i).or_insert(0) += 1
    }
    h
}

fn part2(x: &str) -> i32 {
    let (list1, list2) = parse(x);
    let h = tally(&list2);
    list1.iter().fold(0, |mut acc, x| {
        // I prefer the match arms over HashMap.get().unwrap_or() because of
        // all those tricky pointers in the one-liner.
        let y = match h.get(x) {
            Some(&y) => y,
            None => 0
        };
        acc += x * y;
        //acc += x * (*h.get(&x).unwrap_or(&0)); // yuck.
        acc
    })
}

#[cfg(test)]
mod day01 {
    use super::*;

    const SAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test1() {
        assert_eq!(part1(SAMPLE), 11)
    }
 
    #[test]
    fn test2() {
        assert_eq!(part2(SAMPLE), 31)
    }   
}
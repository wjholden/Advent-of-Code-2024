use std::{fs, iter::zip};

fn main() {
    let puzzle = fs::read_to_string("day02.txt").unwrap();
    let p = parse(&puzzle.trim());
    println!("Part 1: {}", part1(&p));
    println!("Part 2: {}", part2_so_annoyed(&p));
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    input.split("\n").map(|line| {
        line.split_whitespace().map(|e| {
            e.parse().unwrap()
        }).collect()
    }).collect()
}

fn is_pair_safe(slope: i32, x1: i32, x2: i32) -> bool {
    let dr = x2 - x1;
    !(slope * dr <= 0 || dr.abs() < 1 || dr.abs() > 3)
}

fn is_row_safe(row: &[i32]) -> bool {
    let slope = row.last().expect("last") - row.first().expect("first");
    for (&x1,&x2) in zip(&row[0..row.len()-1], &row[1..]) {
        if !is_pair_safe(slope, x1, x2) {
            return false
        }
    }
    true
}

fn part1(input: &Vec<Vec<i32>>) -> usize {
    input.iter().filter(|&row| is_row_safe(row)).count()
}

// Geez. The stupid approach actually works. What's that thing they say about
// premature optimization?
fn part2_so_annoyed(input: &Vec<Vec<i32>>) -> usize {
    input.iter().filter(|&row| {
        for i in 0..row.len() {
            let left = &row[..i];
            let right = &row[i+1..];
            let row_minus_one = [left,right].concat();
            if is_row_safe(&row_minus_one) {
                return true
            }
        }
        false
    }).count()
}

#[cfg(test)]
mod day02 {
    use std::assert_eq;

    use super::*;

    const SAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test1() {
        let s = parse(SAMPLE);
        assert_eq!(part1(&s), 2)
    }

    #[test]
    fn test2() {
        let s = parse(SAMPLE);
        assert_eq!(part2_so_annoyed(&s), 4)
    }
}
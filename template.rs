use std::time::Instant;

fn main() {
    println!("Day 99");
    let start = Instant::now();
    let puzzle = include_str!("../../puzzles/day99.txt");
    println!("Part 1: {}", part1(&puzzle));
    // println!("Part 2: {}", part2(&puzzle));
    println!("Time: {} milliseconds", start.elapsed().as_millis());
}

fn part1(input: &str) -> usize {
    0
}

#[cfg(test)]
mod day99 {
    use std::assert_eq;

    use super::*;

    const SAMPLE: &str = "";

    #[test]
    fn test1() {
        assert_eq!(part1(SAMPLE), 0)
    }
 
    #[test]
    fn test2() {
        //assert_eq!(part2(SAMPLE), 0)
    }   
}
fn main() {
    let puzzle = include_str!("../../puzzles/day13.txt");
    println!("{}", puzzle);
    println!("Part 1: {}", part1(&puzzle));
    //println!("Part 2: {}", part2(&puzzle));
}

fn part1(_input: &str) -> i32 {
    0
}

#[cfg(test)]
mod day13 {
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
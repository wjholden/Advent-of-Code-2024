use std::fs;

fn main() {
    let puzzle = fs::read_to_string("dayXX.txt").unwrap();
    println!("{}", puzzle);
    //println!("Part 1: {}", part1(&puzzle));
    //println!("Part 2: {}", part2(&puzzle));
}

#[cfg(test)]
mod day_xx {
    //use std::assert_eq;

    //use super::*;

    //const SAMPLE: &str = "";

    #[test]
    fn test1() {
        //assert_eq!(part1(SAMPLE), 0)
    }
 
    #[test]
    fn test2() {
        //assert_eq!(part2(SAMPLE), 0)
    }   
}
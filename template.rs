fn main() {
    let puzzle = include_str!("../../puzzles/day99.txt").trim();
    println!("Part 1: {}", part1(&puzzle));
    // println!("Part 2: {}", part2(&puzzle));
}

fn part1(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod day99 {
    use super::*;

    const SAMPLE: &str = "";

    #[test]
    fn test1() {
        assert_eq!(part1(SAMPLE), 0)
    }  
}
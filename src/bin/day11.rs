use std::collections::HashMap;

fn main() {
    let puzzle = include_str!("../../puzzles/day11.txt");
    println!("Part 1: {}", part1(puzzle, 25));
    println!("Part 2: {}", part2(puzzle, 75));
}

fn part1(input: &str, blinks: u64) -> usize {
    let mut stones: Vec<u64> = input.split_whitespace().map(|s| {
        s.parse::<u64>().unwrap()
    }).collect();

    for _ in 0..blinks {
        let mut v = vec![];
        for x in stones.into_iter() {
            match x {
                0 => v.push(1),
                x if (x.ilog10() + 1) % 2 == 0 => {
                    let digits = 1 + x.ilog10();
                    let left = x / 10_u64.pow(digits/2);
                    let right = x % 10_u64.pow(digits/2);
                    v.push(left);
                    v.push(right);
                },
                x => v.push(x * 2024)
            }
        };
        stones = v;
    }
    stones.len()
}

// I took inspiration from https://www.reddit.com/r/adventofcode/comments/1hbm0al/comment/m1hr2p6/
// This whole thing is really a counting problem. A recursive solution isn't easy to memoize
// (maybe impossible?) because the subtrees aren't of equal size, and it's hard to know
// when you've reached the base case where you can stop early. So, if you start from 0,
// you go to 1, to 2024, and then start branching. It looks like some branches do not
// attract to some common value, so you end up with partially-overlapping subtrees that
// you can't count until you have the whole thing.
fn part2(input: &str, blinks: u8) -> u64 {
    let mut stones: HashMap<u64, u64> = input.split_whitespace().map(|s| {
        (s.parse::<u64>().unwrap(), 1)
    }).collect();

    for _ in 0..blinks {
        let mut y = HashMap::new();
        stones.into_iter().for_each(|(k,v)| {
            match k {
                0 => *y.entry(1).or_insert(0) += v,
                k if (k.ilog10() + 1) % 2 == 0 => {
                    let digits = 1 + k.ilog10();
                    *y.entry(k / 10_u64.pow(digits/2)).or_insert(0) += v;
                    *y.entry(k % 10_u64.pow(digits/2)).or_insert(0) += v;
                },
                k => *y.entry(k * 2024).or_insert(0) += v,
            };
        });
        stones = y;
    }

    stones.values().sum()
}

#[cfg(test)]
mod day11 {
    use std::assert_eq;

    use super::*;

    const SAMPLE: &str = "125 17";

    #[test]
    fn test1() {
        assert_eq!(part1(SAMPLE, 6), 22);
    }
 
    #[test]
    fn test2() {
        assert_eq!(part1(SAMPLE, 25), 55312);
    }   

    #[test]
    fn test3() {
        assert_eq!(part2(SAMPLE, 6), 22);
    }
 
    #[test]
    fn test4() {
        assert_eq!(part2(SAMPLE, 25), 55312);
    } 
}
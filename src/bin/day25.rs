use itertools::Itertools;

fn main() {
    let puzzle = include_str!("../../puzzles/day25.txt");
    println!("Part 1: {}", part1(&puzzle));
}

fn part1(input: &str) -> usize {
    let (locks, keys) = parse(input);
    locks.iter().cartesian_product(keys.iter()).filter(|(lock, key)| {
        match (lock, key) { 
            (KL::Lock(l), KL::Key(k)) => {
                for i in 0..5 {
                    if l[i] < k[i] {
                        return false
                    }
                }
            }
            _ => unreachable!(),
        }
        true
    }).count()
}

#[derive(Debug)]
enum KL {
    Key([usize; 5]),
    Lock([usize; 5]),
}

fn parse(input: &str) -> (Vec<KL> , Vec<KL>) {
    let mut locks = vec![];
    let mut keys = vec![];
    let groups = input.split("\n\n");
    for group in groups {
        let mut h = [0,0,0,0,0];
        let rows: Vec<&str> = group.lines().collect();
        let is_lock = rows[0] == "#####";
        let sentinel = match is_lock {
            true => '.',
            false => '#'
        };
        for (i, row) in group.lines().enumerate() {
            let height = if is_lock {
                7 - i
            } else {
                7 - i
            };
            for (j, c) in row.char_indices() {
                if c == sentinel && height > h[j] {
                    h[j] = height;
                }
            }   
        }
        match is_lock {
            true => locks.push(KL::Lock(h)),
            false => keys.push(KL::Key(h))
        }
    }
    (locks, keys)
}

#[cfg(test)]
mod day25 {
    use std::assert_eq;

    use super::*;

    const SAMPLE: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    #[test]
    fn test1() {
        assert_eq!(part1(SAMPLE), 3)
    } 
}
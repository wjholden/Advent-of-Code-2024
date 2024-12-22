use core::panic;
use std::collections::HashMap;

use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use serde::Deserialize;

fn main() {
    let puzzle = include_str!("../../puzzles/day21.txt");
    println!("Part 1: {}", part1(&puzzle));
    // println!("Part 2: {}", part2(&puzzle));
}

#[derive(Debug, Deserialize)]
struct Key {
    label: char,
    neighbors: Vec<Neighbor>
}

#[derive(Debug, Deserialize)]
struct Neighbor {
    label: char,
    direction: char
}

#[derive(Debug)]
struct Robot {
    position: char,
    edges: HashMap<char, Vec<char>>, // adjacency list for graph exploration
    directions: HashMap<(char, char), char>,
}

impl Robot {
    fn new(keypad: &str) -> Robot {
        let keypad: Vec<Key> = serde_json::from_str(keypad).unwrap();
        let mut edges = HashMap::new();
        let mut directions = HashMap::new();
        for key in keypad {
            let mut adj = vec![];
            for neighbor in key.neighbors {
                adj.push(neighbor.label);
                directions.insert((key.label, neighbor.label), neighbor.direction);
            }
            edges.insert(key.label, adj);
        }
        Robot {
            position: 'A',
            edges, 
            directions
        }
    }

    fn new_directional() -> Robot {
        Robot::new(include_str!("directional_keypad.json"))
    }

    fn new_numeric() -> Robot {
        Robot::new(include_str!("numeric_keypad.json"))
    }

    fn goto(&mut self, destination: char) -> Vec<char> {
        let start = self.position;
        if !self.edges.contains_key(&destination) {
            panic!("Destination {destination} is not in our keypad ({:?}).", self.edges.keys());
        }
        let successors = |x: &char| {
            let mut adj = vec![];
            for neighbor in self.edges.get(x).unwrap() {
                // This is probably where we can optimize the solution.
                // Right now we're treating each node as equal cost, but really
                // we need to favor consecutive button presses as much as possible.
                adj.push((*neighbor, 1))
            }
            adj
        };

        let (path, _length) = dijkstra(&start, successors, |&x| x == destination).unwrap();
        let mut instructions = vec![];
        for i in 1..path.len() {
            let a = path[i-1];
            let b = path[i];
            instructions.push(*self.directions.get(&(a,b)).unwrap());
        }

        instructions.sort();

        // https://www.reddit.com/r/adventofcode/comments/1hj4d0c/comment/m36s61s/?context=3
        // 
        // Consider a solution like this:
        // https://www.reddit.com/r/adventofcode/comments/1hja685/comment/m35rvek/
        // where we just manually store all possible moves in our database and
        // don't both with programmatic pathfinding.
        // I'm so annoyed with this problem.
        let mut vertical_first = false;
        if (self.position == '0' || self.position == 'A') &&
            (destination == '1' || destination == '4' || destination == '7') {
                vertical_first = true;
        }
        if instructions.contains(&'v') && instructions.contains(&'>') {
            vertical_first = true;
        }
        if vertical_first {
            instructions.reverse();
        }

        instructions.push('A');
        // contradictions
        assert!(!(instructions.contains(&'v') && instructions.contains(&'^')));
        assert!(!(instructions.contains(&'<') && instructions.contains(&'>')));
        self.position = destination;
        instructions
    }
}

// https://www.reddit.com/r/adventofcode/comments/1hj7f89/2024_day_21_part_1_found_a_rule_to_make_it_work/
// https://www.reddit.com/r/adventofcode/comments/1hja685/2024_day_21_here_are_some_examples_and_hints_for/
fn part1(input: &str) -> usize {
    let full_lines = parse(input);

    let mut numpad = Robot::new_numeric();
    let mut dpad_1 = Robot::new_directional();
    let mut dpad_2 = Robot::new_directional();

    let mut complexity = 0;

    // Part 2: memoize (sequence,depth) -> ?? 
    // https://www.reddit.com/r/adventofcode/comments/1hj2odw/comment/m38fg11/
    //
    // Another part 2 solution by others:
    // https://www.reddit.com/r/adventofcode/comments/1hj2odw/comment/m37qo4f/

    for line in full_lines {        
        let l0 = line.chars();
        let l1 = l0.into_iter().flat_map(|c| numpad.goto(c));
        let l2 = l1.into_iter().flat_map(|c| dpad_1.goto(c));
        let l3 = l2.into_iter().flat_map(|c| dpad_2.goto(c));
        let result = l3.collect_vec();
        let length = result.len();

        let num: usize = line[0..3].parse().unwrap();
        complexity += num * length;
        //println!("{line} (complexity: {} * {})", length, num);
    }

    complexity
}

fn parse(input: &str) -> Vec<&str> {
    input.trim().lines().collect()
}

#[cfg(test)]
mod day21 {
    use std::assert_eq;

    use super::*;

    const SAMPLE: &str = "029A
980A
179A
456A
379A";

    #[test]
    fn test1() {
        assert_eq!(part1(SAMPLE), 126384)
    }

    #[test]
    fn test029a() {
        assert_eq!(part1("029A"), 68 * 29)
    }

    #[test]
    fn test980a() {
        assert_eq!(part1("980A"), 60 * 980)
    }

    #[test]
    fn test179a() {
        assert_eq!(part1("179A"), 68 * 179)
    }

    #[test]
    fn test456a() {
        assert_eq!(part1("456A"), 64 * 456)
    }

    #[test]
    fn test379a() {
        assert_eq!(part1("379A"), 64 * 379)
    }

    //#[test]
    //fn test2() {
        //assert_eq!(part2(SAMPLE), 0)
    //}   
}
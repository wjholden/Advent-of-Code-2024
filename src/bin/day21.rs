use core::panic;
use std::collections::HashMap;

use pathfinding::prelude::dijkstra;
use serde::Deserialize;

fn main() {
    println!("Day 21");
    let puzzle = include_str!("../../puzzles/day21.txt");
    println!("Part 1: {}", part1(&puzzle)); // 133054 too high
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
        self.position = destination;
        let mut instructions = vec![];
        for i in 1..path.len() {
            let a = path[i-1];
            let b = path[i];
            instructions.push(*self.directions.get(&(a,b)).unwrap());
        }

        instructions.sort();
        if instructions.contains(&'<') {
            instructions.reverse();
        }

        instructions.push('A');
        // contradictions
        assert!(!(instructions.contains(&'v') && instructions.contains(&'^')));
        assert!(!(instructions.contains(&'<') && instructions.contains(&'>')));
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

    //println!("Numpad: {numpad:?}");
    //println!("1st D-Pad: {dpad_1:?}");
    //println!("2nd D-Pad: {dpad_2:?}");

    let mut complexity = 0;

    for line in full_lines {        
        let mut s1 = String::new();
        let mut s2 = String::new();
        let mut s3 = String::new();
        let mut length = 0;

        for dst in line.chars() {
            for dst in numpad.goto(dst) {
                s1.push(dst);
                for dst in dpad_1.goto(dst) {
                    s2.push(dst);
                    for dst in dpad_2.goto(dst) {
                        s3.push(dst);
                        length += 1;
                    }   
                }
            }
            s3.push(' ');
            s2.push(' ');
            s1.push(' ');
        }
        let num: usize = line[0..3].parse().unwrap();
        complexity += num * length;
        println!("{s3}");
        println!("{s2}");
        println!("{s1}");
        println!("{line} (complexity: {} * {})", length, num);
    }

    let mut test = ['<', '>', 'v', '^'];
    test.sort();
    println!("{test:?}");

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

    //#[test]
    //fn test2() {
        //assert_eq!(part2(SAMPLE), 0)
    //}   
}
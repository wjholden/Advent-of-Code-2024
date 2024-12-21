use std::collections::{BTreeMap, HashMap, VecDeque};

fn main() {
    println!("Day 21");
    let puzzle = include_str!("../../puzzles/day21.txt");
    println!("Part 1: {}", part1(&puzzle));
    // println!("Part 2: {}", part2(&puzzle));
}

#[derive(Debug)]
struct Keypad {
    buttons: BTreeMap<char, (i8, i8)>, // (key, (row, col))
    //cache: HashMap<(char,char), Vec<char>>, // ((src key, dst key), [path])
    position: char,
}

impl Keypad {
    fn new(layout: Vec<(char, (i8, i8))>) -> Keypad {
        let buttons = BTreeMap::from_iter(layout);
        //let directions = HashMap::new();
        Keypad { buttons, /*cache: HashMap::new(),*/ position: 'A' }
    }

    pub fn new_numeric() -> Keypad {
        Keypad::new(vec![
            ('7', (0, 0)),
            ('8', (0, 1)),
            ('9', (0, 2)),
            ('4', (1, 0)),
            ('5', (1, 1)),
            ('6', (1, 2)),
            ('1', (2, 0)),
            ('2', (2, 1)),
            ('3', (2, 2)),
            ('0', (3, 1)),
            ('A', (3, 2)),
        ])
    }

    pub fn new_directional() -> Keypad {
        Keypad::new(vec![
            ('^', (0, 1)),
            ('A', (0, 2)),
            ('<', (1, 0)),
            ('v', (1, 1)),
            ('>', (1, 2)),
        ])
    }

    pub fn goto(&mut self, destination: char) -> Vec<char> {
        // perform an ugly BFS to find the button. We can probably memoize this later.
        let mut frontier = VecDeque::new();
        frontier.push_back(self.position);
        let mut path = HashMap::new();
        let buttons_inv: HashMap<(i8, i8), char> = self.buttons.iter().map(
            |(&key, &value)| (value, key)).collect();
        let directions = HashMap::from([
            ((-1, 0), '^'),
            ((0, 1), '>'),
            ((1, 0), 'v'),
            ((0, -1), '<'),
        ]);

        loop {
            let current = frontier.pop_front().unwrap();
            if current == destination {
                break
            }
            let (row, col) = *self.buttons.get(&current).unwrap();

            for ((dr, dc), label) in directions.iter() {
                if let Some(&neighbor) = buttons_inv.get(&(row+dr, col+dc)) {
                    if !path.contains_key(&neighbor) {
                        path.insert(neighbor, (current, *label));
                        frontier.push_back(neighbor);
                    }
                }
            }
            //println!("from {current} we can explore {frontier:?}");
        }

        let mut unwind = destination;
        let mut sequence = vec![];
        while unwind != self.position {
            let (parent, label) = *path.get(&unwind).unwrap();
            sequence.push(label);
            unwind = parent;
        }
        self.position = destination;
        
        // check for contradictions
        assert!(!(sequence.contains(&'<') && sequence.contains(&'>')));
        assert!(!(sequence.contains(&'^') && sequence.contains(&'v')));

        sequence.sort();
        //sequence.reverse();
        sequence.push('A');
        sequence
    }
}

fn part1(input: &str) -> usize {
    let full_lines = parse(input);

    let mut numpad = Keypad::new_numeric();
    let mut dpad_1 = Keypad::new_directional();
    let mut dpad_2 = Keypad::new_directional();
    let mut complexity = 0;

    for line in full_lines {        
        let mut s1 = String::new();
        let mut s2 = String::new();
        let mut s3 = String::new();

        for dst in line.chars() {
            for dst in numpad.goto(dst) {
                s1.push(dst);
                for dst in dpad_1.goto(dst) {
                    s2.push(dst);
                    for dst in dpad_2.goto(dst) {
                        s3.push(dst);
                    }
                }
            }
        }
        let num: usize = line[0..3].parse().unwrap();
        complexity += num * s3.len();
        println!("{s3}");
        println!("{s2}");
        println!("{s1}");
        println!("{line} (complexity: {} * {})", s3.len(), num);
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
 
    //#[test]
    //fn test2() {
        //assert_eq!(part2(SAMPLE), 0)
    //}   
}
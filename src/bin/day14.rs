use itertools::Itertools;
use regex::Regex;
use lazy_static::lazy_static;

fn main() {
    let puzzle = include_str!("../../puzzles/day14.txt");
    println!("Part 1: {}", part1(puzzle, 101, 103));
    println!("Part 2: {}", part2(puzzle, 101, 103));
}

fn part1(input: &str, width: i16, height: i16) -> i64 {
    let mut robots = input.trim().split('\n').map(Robot::new).collect_vec();
    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.tick(width, height);
        }
    }
    safety_factor(&robots, width, height)
}

fn part2(input: &str, width: i16, height: i16) -> i16 {
    let mut robots = input.trim().split('\n').map(Robot::new).collect_vec();
    let mut i = 0;
    
    // The Christmas Tree appears when all of the robots have distinct positions.
    // 
    // In retrospect, it should have guessed that the period length is 101 * 103 = 10403.
    while intersections(&robots) != robots.len() && i < width * height {
        for robot in robots.iter_mut() {
            robot.tick(width, height);
        }
        i += 1;
    }
    println!("{}", tree(&robots, width, height));
    i
}

fn safety_factor(robots: &[Robot], width: i16, height: i16) -> i64 {
    let mut quadrants = [0, 0, 0, 0, 0];
    for robot in robots.iter() {
        quadrants[robot.quadrant(width, height)] += 1
    }
    quadrants[1..].iter().product()
}

fn intersections(robots: &[Robot]) -> usize {
    robots.iter().map(|robot| (robot.x, robot.y)).unique().count()
}

fn tree(robots: &[Robot], width: i16, height: i16) -> String {
    let mut s = (0..height).map(|_|
        str::repeat(" ", width as usize)
    ).collect_vec();
    for robot in robots {
        let row = robot.y as usize;
        let col = robot.x as usize;
        s[row].replace_range(col..col+1, "*");
    }
    s.join("\n")
}

#[derive(Debug)]
struct Robot {
    x: i16,
    y: i16,
    dx: i16,
    dy: i16
}

impl Robot {
    fn new(s: &str) -> Robot {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"p=(?P<x>\d+),(?P<y>\d+) v=(?P<dx>-?\d+),(?P<dy>-?\d+)").unwrap();
        }
        let captures = RE.captures(s).unwrap();
        let x = captures.name("x").unwrap().as_str().parse::<i16>().unwrap();
        let y = captures.name("y").unwrap().as_str().parse::<i16>().unwrap();
        let dx = captures.name("dx").unwrap().as_str().parse::<i16>().unwrap();
        let dy = captures.name("dy").unwrap().as_str().parse::<i16>().unwrap();
        Robot { x, y, dx, dy }
    }

    fn tick(&mut self, width: i16, height: i16) {
        let x = (self.x + self.dx + width) % width;
        let y = (self.y + self.dy + height) % height;

        assert!(x >= 0);
        assert!(x < width);
        assert!(y >= 0);
        assert!(y < height);

        self.x = x;
        self.y = y;
    }

    fn quadrant(&self, width: i16, height: i16) -> usize {
        match (self.x.cmp(&(width/2)), self.y.cmp(&(height/2))) {
            (std::cmp::Ordering::Equal, _) => 0,
            (_, std::cmp::Ordering::Equal) => 0,
            (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => 1,
            (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => 2,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => 3,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => 4,
        }
    }
}

#[cfg(test)]
mod day14 {
    use std::assert_eq;

    use super::*;

    const SAMPLE: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test1() {
        assert_eq!(part1(SAMPLE, 11, 7), 12);

        part2(SAMPLE, 11, 7);
    } 
}
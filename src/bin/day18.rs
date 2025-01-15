use std::collections::BTreeSet;

use pathfinding::prelude::dijkstra;

fn main() {
    let puzzle = include_str!("../../puzzles/day18.txt");
    let obstacles = parse(puzzle);
    let (_path, distance) = part1(&obstacles, 1024, (70,70)).unwrap();
    println!("Part 1: {}", distance);
    println!("Part 2: {}", part2(&obstacles, 1024, (70,70)));
}

fn part1(obstacles: &[(usize,usize)], limit: usize, target: (usize, usize)) -> Option<(Vec<(usize,usize)>,usize)> {
    let obstacles = BTreeSet::from_iter(obstacles[..limit].iter());
    
    let successor = |position: &(usize, usize)| {
        let (row, col) = *position;
        let mut successors = vec![];
        if 0 < row && !obstacles.contains(&(row-1,col)) {
            successors.push(((row-1,col),1));
        }
        if row < target.0 && !obstacles.contains(&(row+1,col)) {
            successors.push(((row+1,col),1));
        }
        if 0 < col && !obstacles.contains(&(row,col-1)) {
            successors.push(((row,col-1),1));
        }
        if col < target.1 && !obstacles.contains(&(row,col+1)) {
            successors.push(((row,col+1),1));
        }
        successors
    };

    dijkstra(&(0,0), successor, |position| *position == target)
}

fn part2(obstacles: &[(usize,usize)], limit: usize, target: (usize, usize)) -> String {
    let mut left = limit;
    let mut right = obstacles.len()-1;
    loop {
        let midpoint = (left + right)/2;
        if left + 1 == right {
            break
        }
        if part1(obstacles, midpoint, target).is_none() {
            right = midpoint;
        } else {
            left = midpoint;
        }
    }
    let (row,col) = obstacles[left];
    format!("{row},{col}")
}

fn parse(input: &str) -> Vec<(usize,usize)> {
    input.trim().split('\n').map(|line| {
        let mut xy = line.split(',');
        let x = xy.next().unwrap().parse().unwrap();
        let y = xy.next().unwrap().parse().unwrap();
        (x,y)
    }).collect()
}

#[cfg(test)]
mod day18 {
    use std::assert_eq;

    use super::*;

    const SAMPLE: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    const SAMPLE_TARGET: (usize, usize) = (6, 6);

    const SAMPLE_LIMIT: usize = 12;

    #[test]
    fn test1() {
        let sample_input = parse(SAMPLE);
        assert_eq!(part1(&sample_input, SAMPLE_LIMIT, SAMPLE_TARGET).unwrap().1, 22)
    }
 
    #[test]
    fn test2() {
        let sample_input = parse(SAMPLE);
        assert_eq!(part2(&sample_input, SAMPLE_LIMIT, SAMPLE_TARGET), "6,1")
    }   
}
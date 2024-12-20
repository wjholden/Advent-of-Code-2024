use std::collections::HashMap;
use grid::*;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let puzzle = include_str!("../../puzzles/day20.txt");
    println!("Part 1: {}", part1(&puzzle, 100, 2)); // 1511
    println!("Part 2: {}", part1(&puzzle, 100, 20));
    // println!("Part 2: {}", part2(&puzzle));
}

fn part1(input: &str, picoseconds_saved: usize, cheat_length: usize) -> usize {
    let (grid, start, end) = parse(input);

    // The problem is simpler than it appears because there are no branches.
    let mut path = HashMap::new();
    path.insert(start, 0);
    let mut position = start;
    let mut distance = 0;
    while position != end {
        // There should be only one '.' around us that hasn't been visited.
        let (row, col) = position;
        for neighbor in adjacencies4(row, col, grid.rows(), grid.cols(), 1) {
            if grid[neighbor] == '.' && !path.contains_key(&neighbor) {
                position = neighbor;
                distance += 1;
                continue
            }
        }
        path.insert(position, distance);
    }

    path.par_iter().map(|(position, d1)| {
        let (row, col) = *position;
        let neighbors = adjacencies(row, col, grid.rows(), grid.cols(), cheat_length);
        neighbors.into_iter().filter(|neighbor| {
            match path.get(&neighbor) {
                Some(&d2) => {
                    if d2 > d1 + 2 && d2 - (d1 + manhattan_distance(position, &neighbor)) >= picoseconds_saved {
                        true
                    } else {
                        false
                    }
                },
                None => false
            }
        }).count()
    }).sum()
}

fn manhattan_distance(p1: &(usize, usize), p2: &(usize, usize)) -> usize {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

fn adjacencies4(row: usize, col: usize, rows: usize, cols: usize, delta: usize) -> Vec<(usize, usize)> {
    let mut adj = vec![];
    if 0 + delta <= row {
        adj.push((row-delta, col));
    }
    if row < rows - delta {
        adj.push((row+delta, col));
    }
    if 0 + delta <= col {
        adj.push((row, col-delta));
    }
    if col < cols - delta {
        adj.push((row, col+delta));
    }
    adj
}

fn adjacencies(row: usize, col: usize, rows: usize, cols: usize, delta: usize) -> Vec<(usize, usize)> {
    let mut adj = vec![];
    let row = row as i32;
    let col = col as i32;
    let rows = rows as i32;
    let cols = cols as i32;
    let delta = delta as i32;

    let top = 0.max(row - delta);
    let bottom = (rows-1).min(row + delta);
    let left = 0.max(col - delta);
    let right = (cols-1).min(col + delta);

    //println!("At {row},{col} our boundary box is {top} above, {bottom} below, {left} to the left and {right} to the right.");

    for i in top..=bottom {
        for j in left..=right {
            let distance = i.abs_diff(row) + j.abs_diff(col);
            //println!("{i},{j} distance is {distance}");
            if /*1 < distance &&*/ distance <= delta as u32 {
                adj.push((i as usize, j as usize));
            }
        }
    }

    adj.sort();
    adj
}

fn parse(input: &str) -> (Grid<char>, (usize, usize), (usize, usize)) {
    let mut grid = grid![];
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (row, line) in input.lines().enumerate() {
        grid.push_row(line.chars().enumerate().map(|(col, c)| {
            match c {
                c @ ('#' | '.') => c,
                'S' => {
                    start = (row, col);
                    '.'
                },
                'E' => {
                    end = (row, col);
                    '.'
                },
                _ => unreachable!()
            }
        }).collect_vec());
    };
    (grid, start, end)
}

#[cfg(test)]
mod day20 {
    use std::assert_eq;

    use super::*;

    const SAMPLE: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    const TINY: &str = "#######
#S#...#
#.#.#.#
#...#E#
#######";

    #[test]
    fn tiny() {
        assert_eq!(part1(TINY, 4, 2), 2)
    }

    #[test]
    fn test1() {
        assert_eq!(part1(SAMPLE, 64, 2), 1)
    }
 
    #[test]
    fn test2() {
        assert_eq!(part1(SAMPLE, 74, 20), 4+3);
        assert_eq!(part1(SAMPLE, 76, 20), 3)
    }   
}
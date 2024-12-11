use std::{collections::{HashMap, HashSet, VecDeque}, fs};

use grid::*;

fn main() {
    let puzzle = fs::read_to_string("puzzles/day10.txt").unwrap();
    let result = explore(&puzzle);
    println!("Part 1: {}", result.0);
    println!("Part 2: {}", result.1);
}

fn parse(input: &str) -> Grid<u32> {
    let mut grid: Grid<u32> = grid![];
    for line in input.trim().split('\n') {
        grid.push_row(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }
    grid
}

fn score(grid: &Grid<u32>, trailhead: (usize, usize)) -> (usize, usize) {
    let mut frontier = VecDeque::new();
    frontier.push_back(trailhead);
    let mut nines = HashSet::new();
    let mut rating = HashMap::new();
    while !frontier.is_empty() {
        if let Some(current) = frontier.pop_front() {
            if grid[current] == 9 {
                nines.insert(current);
                *rating.entry(current).or_insert(0) += 1;
            } else {
                let (x,y) = current;
                let mut neighbors = vec![];
                // There's gotta be a better way. I don't love all this bounds checking.
                if 0 < x {
                    neighbors.push((x-1, y));
                }
                if x < grid.rows() -1 {
                    neighbors.push((x+1, y));
                }
                if 0 < y {
                    neighbors.push((x, y-1));
                }
                if y < grid.cols() - 1 {
                    neighbors.push((x, y+1));
                }
                for (u,v) in neighbors {
                    if grid[(u,v)] == grid[(x,y)] + 1{
                        frontier.push_back((u,v));
                    }
                }
            }
        } else {
            unreachable!()
        }
    }
    (nines.len(), rating.values().sum())
}

fn explore(input: &str) -> (usize, usize) {
    let grid = parse(input);
    grid.indexed_iter().filter_map(|((x,y), value)| match value {
        0 => Some((x,y)),
        _ => None
    }).map(|trailhead| score(&grid, trailhead)).
    fold((0,0), |(cumscore, cumrating), (score, rating)| (cumscore + score, cumrating + rating))
}

#[cfg(test)]
mod day10 {
    use std::assert_eq;

    use super::*;

    const SAMPLE1: &str = "0123
1234
8765
9876";

    const SAMPLE2: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test1() {
        assert_eq!(explore(SAMPLE1).0, 1)
    }

    #[test]
    fn test1_larger() {
        assert_eq!(explore(SAMPLE2).0, 36)
    }

    #[test]
    fn test2() {
        assert_eq!(explore(SAMPLE2).1, 81)
    }   
}
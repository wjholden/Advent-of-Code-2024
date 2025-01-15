
use itertools::Itertools;
use num::complex::Complex;
use grid::*;
use pathfinding::prelude::astar_bag_collect;

fn main() {
    let puzzle = include_str!("../../puzzles/day16.txt");
    let result = solve(puzzle).unwrap();
    println!("Part 1: {}", result.0);
    println!("Part 2: {}", result.1);
}

fn solve(input: &str) -> Result<(i32, usize), ()> {
    let mut grid = grid![];
    for line in input.lines() {
        grid.push_row(line.chars().collect());
    }
    let grid = grid;

    let (startr, startc) = (grid.rows() - 2, 1);
    let (endr, endc) = (1, grid.cols() - 2);
    let start = PD {
        position: Complex::new(startr as i32, startc),
        direction: Complex::new(0, 1), // start facing east
    };
    let successors = |pd: &PD| pd.successors(&grid);
    let end = Complex::new(endr, endc as i32);
    let heuristic = |pd: &PD| (pd.position - end).l1_norm();
    //let heuristic = |_pd: &PD| 1; // this also works
    let success = |pd: &PD| pd.position == end;

    if let Some((paths, path_length)) = astar_bag_collect(&start, successors, heuristic, success) {
        let on_path = paths.into_iter().flatten().map(|pd| {
           (pd.position.re, pd.position.im) 
        }).unique().count();
        Ok((path_length, on_path))
    } else {
        Err(())
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct PD { // position-direction. Is this a case for quaternions?
    position: Complex<i32>,
    direction: Complex<i32>,
}

impl PD {
    fn successors(&self, grid: &Grid<char>) -> Vec<(Self, i32)> {
        let turn_left = Complex::new(0,1);
        let turn_right = Complex::new(0,-1);
        let mut e = vec![];
        e.push((PD {
            position: self.position,
            direction: self.direction * turn_left
        }, 1000));
        e.push((PD {
            position: self.position,
            direction: self.direction * turn_right
        }, 1000));
        let next_pos = self.position + self.direction;
        let (row,col) = (next_pos.re as usize, next_pos.im as usize);
        if row < grid.rows() && col < grid.cols() && grid[(row,col)] != '#' {
            e.push((PD {
                position: next_pos,
                direction: self.direction
            }, 1));
        }
        e
    }
}

#[cfg(test)]
mod day16 {
    use std::assert_eq;

    use super::*;

    const SAMPLE: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const SAMPLE2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn test1() {
        assert_eq!(solve(SAMPLE).unwrap().0, 7036)
    }
 
    #[test]
    fn test2() {
        assert_eq!(solve(SAMPLE).unwrap().1, 45)
    }   

    #[test]
    fn test3() {
        assert_eq!(solve(SAMPLE2), Ok((11048, 64)))
    }
}
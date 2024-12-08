use std::{collections::HashSet, fs};
use grid::*;
use itertools::Itertools;

fn main() {
    let puzzle = fs::read_to_string("puzzles/day08.txt").unwrap();
    let solution = part1(&puzzle);
    println!("Part 1: {}", solution.0);
    println!("Part 1: {}", solution.1);
}

fn parse(input: &str) -> Grid<char> {
    let mut grid: Grid<char> = grid![];
    for line in input.trim().split('\n') {
        grid.push_row(line.chars().collect());
    }
    grid
}

fn find_antennas(grid: &Grid<char>, frequency: char) -> Vec<(i32,i32)> {
    grid.indexed_iter().filter_map(|((x,y), c)| {
        if *c == frequency {
            Some((x as i32,y as i32))
        } else {
            None
        }
    }).collect()
}

fn get_frequencies(grid: &Grid<char>) -> HashSet<char> {
    grid.iter().filter_map(|&c| {
        if c != '.' {
            Some(c) // this is to coerce a copy
        } else {
            None
        }
    }).dedup().collect()
}

fn part1(input: &str) -> (usize, usize) {
    let grid = parse(input);
    let is_in_bounds = |x,y| {
        0 <= x && x < grid.rows() as i32 && 0 <= y && y < grid.cols() as i32
    };
    let mut antinodes = HashSet::new();
    let mut antinodes2 = HashSet::new();
    for frequency in get_frequencies(&grid) {
        for pair in find_antennas(&grid, frequency).into_iter().combinations(2) {
            let (x1,y1) = pair[0];
            let (x2,y2) = pair[1];

            assert!((x1 != x2) && (y1 != y2));

            let dx = (x2 - x1).abs();
            let dy = (y2 - y1).abs();
            let slope = (x2 - x1)/dx * (y2 - y1)/dy;

            for i in 0.. {
                let dx = dx * i;
                let dy = dy * i;

                let (left, right) = match (dx,dy) {
                    (0,0) => ((x1,y1), (x2,y2)), // for part 2 only
                    (0,_) => ((x1, y1.min(y2) - dy), (x2, y1.max(y2) + dy)),
                    (_,0) => ((x1.min(x2) - dx, y1), (x1.max(x2) + dx, y2)),
                    (dx,dy) if slope == -1 => /* negative slope */
                        ((x1.min(x2) - dx, y1.max(y2) + dy),
                        (x1.max(x2) + dx, y1.min(y2) - dy)),
                    (dx,dy) if slope == 1 => /* positive slope */
                        ((x1.min(x2) - dx, y1.min(y2) - dy),
                        (x1.max(x2) + dx, y1.max(y2) + dy)),
                    _ => unreachable!("we should never have used the default match arm")
                };

                if is_in_bounds(left.0, left.1) {
                    if i == 1 {
                        antinodes.insert(left);
                    }
                    antinodes2.insert(left);
                }

                if is_in_bounds(right.0, right.1) {
                    if i == 1 {
                        antinodes.insert(right);
                    }
                    antinodes2.insert(right);
                }

                if !is_in_bounds(left.0, left.1) && 
                    !is_in_bounds(right.0, right.1) {
                        break
                }
            }
        }
    }
    
    //println!("{}", map(input, &antinodes2));
    (antinodes.len(), antinodes2.len())
}

#[allow(dead_code)]
fn map(input: &str, antinodes: &HashSet<(i32,i32)>) -> String {
    let mut grid = parse(input);
    let mut s = String::new();
    for (x,y) in antinodes.iter() {
        grid[(*x as usize,*y as usize)] = '#';
    }
    for row in grid.iter_rows() {
        for &c in row {
            s.push(c);
        }
        s.push('\n');
    }
    s
}

#[cfg(test)]
mod day08 {
    use std::assert_eq;

    use super::*;

    const SAMPLE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    const SAMPLE2: &str = "..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........";

    const SAMPLE3: &str = "..........
..........
..........
....a.....
........a.
.....a....
..........
......A...
..........
..........";

const SAMPLE4: &str = "..........
..........
..........
..........
........a.
.....a....
..........
..........
..........
..........";

const SAMPLE5: &str = "T....#....
...T......
.T........
..........
..........
..........
..........
..........
..........
..........";

    #[test]
    fn test1() {
        assert_eq!(part1(SAMPLE).0, 14)
    }

    #[test]
    fn small() {
        assert_eq!(part1(SAMPLE2).0, 2)
    }

    #[test]
    fn multi() {
        assert_eq!(part1(SAMPLE3).0, 4)
    }

    #[test]
    fn so_negative() {
        assert_eq!(part1(SAMPLE4).0, 1)
    }
 
    #[test]
    fn test2() {
        assert_eq!(part1(SAMPLE).1, 34)
    }   

    #[test]
    fn t_freq() {
        assert_eq!(part1(SAMPLE5).1, 9)
    }   

    #[test]
    fn grid() {
        let grid = parse(SAMPLE);
        let frequencies = get_frequencies(&grid);
        
        let expected = HashSet::from(['A', '0']);
        assert_eq!(frequencies, expected);
    }
}
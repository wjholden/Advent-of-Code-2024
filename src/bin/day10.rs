use std::{collections::{HashSet, VecDeque}, fs};

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
    let mut rating = 0;
    while !frontier.is_empty() {
        if let Some(current) = frontier.pop_front() {
            if grid[current] == 9 {
                nines.insert(current);
                rating += 1;
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
    (nines.len(), rating)
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

const CHALLENGE: &str = "0123456789876543210123456789876543210
1234567898987654321234567898987654321
2345678987898765432345678987898765432
3456789876789876543456789876789876543
4567898765678987654567898765678987654
5678987654567898765678987654567898765
6789876543456789876789876543456789876
7898765412345678987898765432105678987
8987654301234567898987654321214567898
9876543210123456789876543210123456789
8987654321214567898987654301234567898
7898765432105678987898765432321678987
6789876543456789876789876543210789876
5678987654567898765678987654567898765
4567898765678987654567898765678987654
3456789876789876543456789876789876543
2345678987898765432345678987898765432
1234567898987654321234567898987654321
0123456789876543210123456789876543210
1234567898987654321234567898987654321
2345678987898765410145678987898765432
3456789876789876543456789876789876543
4567898765678987652567898765678987654
5678987654567898761678987654567898765
6789876543456789870789012543456789876
7898765432345678989898123432345678987
8987654321234567898987654321234567898
9876543210123456789876543210123456789
8987654321214567898987654321234567898
7898765432105678987898765432345678987
6789876543456789876789876543456789876
5678987654567898765678987654567898765
4567898765678987654567898765678987654
3456789876789876543456789876789876543
2345678987898765432345678987898765432
1234567898987654321234567898987654321
0123456789876543210123456789876543210";

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

    // https://www.reddit.com/r/adventofcode/comments/1hawlbo/2024_day_10_challenge_input/?share_id=oPVT2vR_GC7TXo4_Jhxk4&utm_content=1&utm_medium=android_app&utm_name=androidcss&utm_source=share&utm_term=2
    #[test]
    fn challenge() {
        assert_eq!(explore(CHALLENGE).1, 16451)
    }
}
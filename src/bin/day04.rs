use std::fs;
use itertools::Itertools;

fn main() {
    let puzzle = fs::read_to_string("puzzles/day04.txt").unwrap();
    println!("Part 1: {}", part1(&puzzle));
    println!("Part 2: {}", part2(&puzzle));
}

// This problem looked like it would be easy, but Rust doesn't support
// lookaheads in its regexes, and searching diagonally isn't obvious either.
fn part1(input: &str) -> i32 {
    let m: Vec<Vec<char>> = input.lines().map(|line| {
        line.chars().collect()      
    }).collect();

    let mut count = 0;
    let rows = m.len() as i32;
    let cols = m[0].len() as i32;

    for (row,col) in (0..rows).cartesian_product(0..cols) {
        count += explore(&m, row, col);
    }
    count
}

fn part2(input: &str) -> i32 {
    let m: Vec<Vec<char>> = input.lines().map(|line| {
        line.chars().collect()      
    }).collect();
    let rows = m.len();
    let cols = m[0].len();
    let mut count = 0;
    for (row, col) in (1..rows-1).cartesian_product(1..cols-1) {
        // Bounds checking shouldn't be necessary now due to offsets.
        if m[row][col] == 'A' {
            let d1 = String::from_iter([m[row-1][col-1], 'A', m[row+1][col+1]]);
            let d2 = String::from_iter([m[row+1][col-1], 'A', m[row-1][col+1]]);
            if (d1 == "MAS" || d1 == "SAM") && (d2 == "MAS" || d2 == "SAM") {
                count += 1
            }
        }
    };
    count
}

fn explore(haystack: &[Vec<char>], row: i32, col: i32) -> i32 {
    let mut count = 0;
    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue
            }
            if search1(haystack, row, col, dr, dc).is_some() {
                count += 1
            }
        }
    }
    count
}

fn search1(haystack: &[Vec<char>], row: i32, col: i32, dr: i32, dc: i32) -> Option<()> {
    let rows = haystack.len() as i32;
    let cols = haystack[0].len() as i32;
    for radius in 0..=3 {
        let r = row + dr * radius;
        if r < 0 || rows <= r {
            return None
        }
        let c = col + dc * radius;
        if c < 0 || cols <= c {
            return None
        }
        let l = haystack[r as usize][c as usize];
        if l != ['X','M','A','S'][radius as usize] {
            return None
        }
    }
    Some(())
}

#[cfg(test)]
mod day04 {
    use super::*;

    const SAMPLE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test1() {
        assert_eq!(part1(SAMPLE), 18)
    }
 
    #[test]
    fn test2() {
        assert_eq!(part2(SAMPLE), 9)
    } 
}
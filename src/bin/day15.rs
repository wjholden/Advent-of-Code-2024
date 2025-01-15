use core::fmt;
use std::collections::{HashSet, VecDeque};

use grid::*;
use itertools::Itertools;

fn main() {
    let puzzle = include_str!("../../puzzles/day15.txt");
    println!("Part 1: {}", part1(puzzle));
    println!("Part 2: {}", part2(puzzle));
}

fn part1(input: &str) -> usize {
    let (mut grid, moves) = parse(input);
    let mut robot = find_robot(&grid);

    for m in moves {
        let (g, swap) = try_swap(grid, robot, &m);
        grid = g;
        if swap {
            robot = m.next(robot);
        }
    }
    gps_score(&grid)
}

fn part2(input: &str) -> usize {
    let (grid, moves) = parse(input);
    let mut warehouse = WideWarehouse::new(grid);

    for m in moves {
        warehouse.tick(&m);
    }

    gps_score2(&warehouse.contents)
}

fn gps_score(grid: &Grid<Content>) -> usize {
    grid.indexed_iter().filter_map(|((row,col),content)| {
        match content {
            Content::Box => Some(row * 100 + col),
            _ => None
        }
    }).sum()
}

fn gps_score2(grid: &Grid<WideContent>) -> usize {
    grid.indexed_iter().filter_map(|((row,col),content)| {
        match content {
            WideContent::LeftBox => {
                // The instructions here were really confusing. I see that I was not the only one:
                // https://www.reddit.com/r/adventofcode/comments/1her9i0/day_15_part_2_clarification_for_day_15_part_2_gps/
                Some(row * 100 + col)
            },
            _ => None
        }
    }).sum()
}

fn try_swap(grid: Grid<Content>, position: (usize, usize), movement: &Direction) -> (Grid<Content>, bool) {   
    if matches!(grid[position], Content::Wall) {
        // Base case: we've hit a wall and have to stop
        (grid, false)
    } else if matches!(grid[position], Content::Empty) {
        // Other base case: we've hit a wall and can do swaps
        //println!("we found an empty spot at position {:?}", position);
        (grid, true)
    } else {
        // Recursive case.
        let (mut grid, swap) = try_swap(grid, movement.next(position), movement);
        if swap {
            // We found an empty spot down the line. Do the swaps.
            grid.swap(position, movement.next(position));
            (grid, true)
        } else {
            // We did not find an empty spot. Do not swap.
            (grid, false)
        }
    }
}

fn find_robot(grid: &Grid<Content>) -> (usize, usize) {
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            if matches!(grid[(row,col)], Content::Robot) {
                return (row,col)
            }
        }
    }
    unreachable!()
}

fn parse(input: &str) -> (Grid<Content>, Vec<Direction>) {
    let split = input.split("\n\n").collect_vec();
    assert_eq!(split.len(), 2);
    let mut grid = grid![];
    for line in split[0].lines() {
        grid.push_row(line.chars().map(|c|
            match c {
                '#' => Content::Wall,
                '.' => Content::Empty,
                'O' => Content::Box,
                '@' => Content::Robot,
                _ => unreachable!()
            }).collect()
        );
    }

    let moves = split[1].chars().filter_map(|c|
        match c {
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            _ => None
        }
    ).collect_vec();
    (grid, moves)
}

enum Content {
    Wall,
    Box,
    Empty,
    Robot
}

enum WideContent {
    Wall,
    LeftBox,
    RightBox,
    Empty,
    Robot
}

struct WideWarehouse {
    robot: (usize, usize),
    contents: Grid<WideContent>
}

impl WideWarehouse {
    fn new(grid: Grid<Content>) -> WideWarehouse {
        let mut widened = grid![];
        let mut robot = (0, 0);
        for (i, row) in grid.iter_rows().enumerate() {
            widened.push_row(row.enumerate().flat_map(|(j, c)| {
                match c {
                    Content::Wall => [WideContent::Wall, WideContent::Wall],
                    Content::Box => [WideContent::LeftBox, WideContent::RightBox],
                    Content::Empty => [WideContent::Empty, WideContent::Empty],
                    Content::Robot => {
                        robot = (i, 2 * j);
                        [WideContent::Robot, WideContent::Empty]
                    },
                }
            }).collect());
        }   
        WideWarehouse{ robot, contents: widened }
    }

    fn tick(&mut self, dir: &Direction) {
        if let Ok((empties, explored)) = self.shiftable(dir) {
            for empty in empties.into_iter().rev() {
                self.shift(empty, &explored, dir);
            }
            self.robot = dir.next(self.robot);
        }
    }

    fn shift(&mut self, position: (usize, usize), explored: &HashSet<(usize,usize)>, dir: &Direction) {
        // we're doing this backwards, starting with the empty spaces at the end.
        let (row, col) = position;
        let next_pos = match dir {
            Direction::Down => (row-1, col),
            Direction::Up => (row+1, col),
            Direction::Left => (row, col+1),
            Direction::Right => (row, col-1),
        };
        if explored.contains(&next_pos) {
            match &self.contents[next_pos] {
                WideContent::Wall | WideContent::Empty => (),
                _ => {
                    self.contents.swap(position, next_pos);
                    self.shift(next_pos, explored, dir);
                }
            }
        }
    }

    #[allow(clippy::type_complexity)] // I don't remember why I used such a
    // complicated type, but it works and I'm not willing to change it now.
    fn shiftable(&self, dir: &Direction) -> Result<(Vec<(usize, usize)>, HashSet<(usize,usize)>), ()> {
        let mut frontier = VecDeque::new();
        let mut explored = HashSet::new();
        let mut empties = vec![];

        frontier.push_back(self.robot);
        loop {
            if let Some(current) = frontier.pop_front() {
                if explored.contains(&current) {
                    continue;
                } else {
                    explored.insert(current);
                }
                let (row,col) = current;
                match (&self.contents[current], dir) {
                    (WideContent::Wall, _) => return Err(()),
                    (WideContent::Empty, _) => {
                        assert!(!empties.contains(&current));
                        empties.push(current)
                    },
                    (_, Direction::Left) => frontier.push_back((row,col-1)),
                    (_, Direction::Right) => frontier.push_back((row,col+1)),
                    (WideContent::LeftBox, Direction::Up) => {
                        frontier.push_front((row,col+1)); // visit our neighbor immediately
                        frontier.push_back((row-1,col));
                        frontier.push_back((row-1,col+1));
                    },
                    (WideContent::LeftBox, Direction::Down) => {
                        frontier.push_front((row,col+1)); // visit our neighbor immediately
                        frontier.push_back((row+1,col));
                        frontier.push_back((row+1,col+1));
                    },
                    (WideContent::RightBox, Direction::Up) => {
                        frontier.push_front((row,col-1)); // visit our neighbor immediately
                        frontier.push_back((row-1,col-1));
                        frontier.push_back((row-1,col));
                    },
                    (WideContent::RightBox, Direction::Down) => {
                        frontier.push_front((row,col-1)); // visit our neighbor immediately
                        frontier.push_back((row+1,col-1));
                        frontier.push_back((row+1,col));
                    },
                    (WideContent::Robot, Direction::Up) => frontier.push_back((row-1,col)),
                    (WideContent::Robot, Direction::Down) => frontier.push_back((row+1,col)),
                }
            } else {
                return Ok((empties, explored))
            }
        }
    }
}

impl fmt::Display for WideWarehouse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.contents.iter_rows() {
            for e in row {
                write!(f, "{}", e)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Content::Wall => "#",
            Content::Box => "O",
            Content::Empty => ".",
            Content::Robot => "@"
        })
    }
}

impl fmt::Display for WideContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            WideContent::Wall => "#",
            WideContent::LeftBox => "[",
            WideContent::RightBox => "]",
            WideContent::Empty => ".",
            WideContent::Robot => "@"
        })
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Direction::Up => "^",
            Direction::Down => "v",
            Direction::Left => "<",
            Direction::Right => ">"
        })
    }
}

impl Direction {
    fn next(&self, position: (usize, usize)) -> (usize, usize) {
        let (row, col) = position;
        match self {
            Direction::Down => (row + 1, col),
            Direction::Up => (row - 1, col),
            Direction::Left => (row, col - 1),
            Direction::Right => (row, col + 1)
        }
    }
}

#[cfg(test)]
mod day15 {
    use std::assert_eq;

    use super::*;

    const SAMPLE: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    const SMALL: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const SAMPLE2: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    const EXTRA_406: &str = "#######
#.....#
#.OO@.#
#.....#
#######

<<";

    const EXTRA_509: &str = "#######
#.....#
#.O#..#
#..O@.#
#.....#
#######

<v<<^";

    const EXTRA_822: &str = "#######
#.....#
#.O.O@#
#..O..#
#..O..#
#.....#
#######

<v<<>vv<^^";

    const EXTRA_511: &str = "#######
#.....#
#.#O..#
#..O@.#
#.....#
#######

<v<^";

    const EXTRA_816: &str = "######
#....#
#.O..#
#.OO@#
#.O..#
#....#
######

<vv<<^";

    const EXTRA_2339: &str = "#######
#...#.#
#.....#
#.....#
#.....#
#.....#
#.OOO@#
#.OOO.#
#..O..#
#.....#
#.....#
#######

v<vv<<^^^^^";

    #[test]
    fn parser() {
        let (grid, moves) = parse(SMALL);
        assert_eq!(grid.rows(), 8);
        assert_eq!(grid.cols(), 8);
        assert_eq!(moves.len(), 15);
    }

    #[test]
    fn small() {
        assert_eq!(part1(SMALL), 2028)
    }

    #[test]
    fn test1() {
        assert_eq!(part1(SAMPLE), 10092)
    }
 
    #[test]
    fn test2() {
        assert_eq!(part2(SAMPLE), 9021)
    }   

    #[test]
    fn path() {
        _ = part2(SAMPLE2)
    }

    #[test]
    fn widen() {
        let (grid, _) = parse(SAMPLE);
        let warehouse = WideWarehouse::new(grid);
        println!("{}", warehouse);
    }

    #[test]
    fn extra1() {
        // https://www.reddit.com/r/adventofcode/comments/1heoj7f/2024_day_15_part_2_more_sample_inputs_to_catch/
        assert_eq!(part2(EXTRA_406), 406)
    }

    #[test]
    fn extra2() {
        // https://www.reddit.com/r/adventofcode/comments/1heoj7f/2024_day_15_part_2_more_sample_inputs_to_catch/
        assert_eq!(part2(EXTRA_509), 509)
    }

    #[test]
    fn extra3() {
        // https://www.reddit.com/r/adventofcode/comments/1heoj7f/comment/m25f7qs/
        assert_eq!(part2(EXTRA_822), 822)
    }

    #[test]
    fn extra4() {
        // https://www.reddit.com/r/adventofcode/comments/1heoj7f/comment/m25g60k/
        assert_eq!(part2(EXTRA_511), 511)
    }

    #[test]
    fn extra5() {
        // https://www.reddit.com/r/adventofcode/comments/1heoj7f/comment/m25g60k/
        assert_eq!(part2(EXTRA_816), 816)
    }

    #[test]
    fn extra6() {
        // https://www.reddit.com/r/adventofcode/comments/1heoj7f/comment/m25pxqg/
        assert_eq!(part2(EXTRA_2339), 2339)
    }
}
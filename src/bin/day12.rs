use grid::*;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

fn main() {
    let puzzle = include_str!("../../puzzles/day12.txt");
    let grid = parse(puzzle);
    let (part1, part2) = solve(&grid);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn solve(grid: &Grid<char>) -> (usize, usize) {
    let mut explored: HashSet<(usize, usize)> = HashSet::new();
    let mut regions = vec![];

    for (row, col) in (0..grid.rows()).cartesian_product(0..grid.cols()) {
        if !explored.contains(&(row, col)) {
            let region = explore(grid, row, col);
            explored.extend(&region);
            regions.push(region);
        }
    }

    let part1 = regions
        .iter()
        .map(|region| area(region) * perimeter(region))
        .sum();
    let part2 = regions
        .iter()
        .map(|region| area(region) * edges(region))
        .sum();

    (part1, part2)
}

fn explore(grid: &Grid<char>, row: usize, col: usize) -> HashSet<(usize, usize)> {
    let mut region = HashSet::new();
    let mut discovered = HashSet::new();
    let mut frontier = VecDeque::new();

    discovered.insert((row, col));
    frontier.push_back((row, col));
    let label = grid[(row, col)];

    while let Some((u, v)) = frontier.pop_front() {
        region.insert((u, v));
        if u > 0
            && grid[(u - 1, v)] == label
            && !discovered.contains(&(u - 1, v))
            && !region.contains(&(u - 1, v))
        {
            discovered.insert((u - 1, v));
            frontier.push_back((u - 1, v));
        }
        if v > 0
            && grid[(u, v - 1)] == label
            && !discovered.contains(&(u, v - 1))
            && !region.contains(&(u, v - 1))
        {
            discovered.insert((u, v - 1));
            frontier.push_back((u, v - 1));
        }
        if u < grid.rows() - 1
            && grid[(u + 1, v)] == label
            && !discovered.contains(&(u + 1, v))
            && !region.contains(&(u + 1, v))
        {
            discovered.insert((u + 1, v));
            frontier.push_back((u + 1, v));
        }
        if v < grid.cols() - 1
            && grid[(u, v + 1)] == label
            && !discovered.contains(&(u, v + 1))
            && !region.contains(&(u, v + 1))
        {
            discovered.insert((u, v + 1));
            frontier.push_back((u, v + 1));
        }
    }

    region
}

fn area(region: &HashSet<(usize, usize)>) -> usize {
    region.len()
}

fn perimeter(region: &HashSet<(usize, usize)>) -> usize {
    let mut perimeter = 0;
    for (x, y) in region {
        // sometimes the strong typing is annoying.
        let x = *x as i32;
        let y = *y as i32;
        // OK WTH...there's some impossible bug here where if you change (-1, 0) to (1, 0)
        // you'll still get the right result, but it fails if you take (-1, 0) out completely. IDK.
        for (dx, dy) in [(1, 0), (-1, 0), (0, -1), (0, 1)] {
            if !region.contains(&((x + dx) as usize, (y + dy) as usize)) {
                perimeter += 1
            }
        }
    }
    perimeter
}

#[derive(PartialEq, Eq)]
enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

fn edges(region: &HashSet<(usize, usize)>) -> usize {
    let mut top = HashSet::new();
    let mut bottom = HashSet::new();
    let mut left = HashSet::new();
    let mut right = HashSet::new();
    for (row, col) in region {
        let row = *row as i32;
        let col = *col as i32;
        for side in [Side::Bottom, Side::Top, Side::Left, Side::Right] {
            let (dr, dc, dst) = match side {
                Side::Top => (-1, 0, &mut top),
                Side::Bottom => (1, 0, &mut bottom),
                Side::Left => (0, -1, &mut left),
                Side::Right => (0, 1, &mut right),
            };
            if (row == 0 && side == Side::Top)
                || (col == 0 && side == Side::Left)
                || !region.contains(&((row + dr) as usize, (col + dc) as usize))
            {
                dst.insert((row + dr, col + dc));
            }
        }
    }

    let mut count = 0;

    let count_edges = move |mut set: HashSet<(i32, i32)>, horizontal| {
        let mut count = 0;
        while !set.is_empty() {
            let origin = *set.iter().next().unwrap();
            set.remove(&origin);
            let (mut r_min, mut c_min) = origin;
            let (mut r_max, mut c_max) = origin;

            if horizontal {
                c_min -= 1;
            } else {
                r_min -= 1;
            }
            while set.contains(&(r_min, c_min)) {
                set.remove(&(r_min, c_min));
                if horizontal {
                    c_min -= 1;
                } else {
                    r_min -= 1;
                }
            }

            if horizontal {
                c_max += 1;
            } else {
                r_max += 1;
            }
            while set.contains(&(r_max, c_max)) {
                set.remove(&(r_max, c_max));
                if horizontal {
                    c_max += 1;
                } else {
                    r_max += 1;
                }
            }
            count += 1;
        }
        count
    };

    count += count_edges(top, true);
    count += count_edges(bottom, true);
    count += count_edges(left, false);
    count += count_edges(right, false);

    count
}

fn parse(input: &str) -> Grid<char> {
    let mut grid: Grid<char> = Grid::default();
    for line in input.lines() {
        grid.push_row(line.chars().collect());
    }
    grid
}

#[cfg(test)]
mod day12 {
    use std::assert_eq;

    use super::*;

    const SAMPLE1: &str = "AAAA
BBCD
BBCC
EEEC";

    const SAMPLE2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    const SAMPLE3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    // https://www.reddit.com/r/adventofcode/comments/1hcib0z/2024_day_12_yet_another_test_case/
    const CHRISTMAS: &str = "XOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOX
OXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXO
XXXXXXXXXXXXXXXMXXXMXEEEEEXRRRRXXRRRRXXYXXXYXXXXXXXXXXXXXXX
OXXXXXXXXXXXXXXMMXMMXEXXXXXRXXXRXRXXXRXYXXXYXXXXXXXXXXXXXXO
XXXXXXXXXXXXXXXMXMXMXEEEEEXRRRRXXRRRRXXXYXYXXXXXXXXXXXXXXXX
OXXXXXXXXXXXXXXMXXXMXEXXXXXRXXXRXRXXXRXXXYXXXXXXXXXXXXXXXXO
XXXXXXXXXXXXXXXMXXXMXEEEEEXRXXXRXRXXXRXXXYXXXXXXXXXXXXXXXXX
OXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXO
XXXXCCCXXHXXXHXRRRRXXIIIIIXXSSSSXTTTTTXMXXXMXXAAAXXXSSSSXXX
OXXCXXXCXHXXXHXRXXXRXXXIXXXSXXXXXXXTXXXMMXMMXAXXXAXSXXXXXXO
XXXCXXXXXHHHHHXRRRRXXXXIXXXXSSSXXXXTXXXMXMXMXAAAAAXXSSSXXXX
OXXCXXXCXHXXXHXRXXXRXXXIXXXXXXXSXXXTXXXMXXXMXAXXXAXXXXXSXXO
XXXXCCCXXHXXXHXRXXXRXIIIIIXSSSSXXXXTXXXMXXXMXAXXXAXSSSSXXXX
OXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXO
XOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOXOX";

    #[test]
    fn test1() {
        assert_eq!(solve(&parse(SAMPLE1)), (140, 80))
    }

    #[test]
    fn test2() {
        assert_eq!(solve(&parse(SAMPLE2)).0, 772)
    }

    #[test]
    fn test3() {
        assert_eq!(solve(&parse(SAMPLE3)), (1930, 1206))
    }

    #[test]
    fn tiny1() {
        assert_eq!(solve(&parse("A")).1, 4)
    }

    #[test]
    fn tiny2() {
        assert_eq!(solve(&parse("BB")).1, 8)
    }

    #[test]
    fn tiny3() {
        assert_eq!(solve(&parse("C\nC")).1, 8)
    }

    #[test]
    fn christmas() {
        assert_eq!(solve(&parse(CHRISTMAS)), (426452, 307122))
    }
}

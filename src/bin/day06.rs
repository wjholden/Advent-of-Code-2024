use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Guard {
    x: i16,
    y: i16,
    dx: i16,
    dy: i16,
}

trait Go {
    fn go(&mut self, world: &World) -> bool;
    fn next(&self) -> (i16, i16);
    fn turn(&mut self);
}

enum MapElement {
    Empty,
    Obstruction,
}

type World = HashMap<(i16, i16), MapElement>;

impl Go for Guard {
    fn go(&mut self, world: &World) -> bool {
        match world.get(&self.next()) {
            Some(MapElement::Empty) => {
                self.x += self.dx;
                self.y += self.dy;
                true
            }
            Some(MapElement::Obstruction) => {
                self.turn();
                true
            }
            None => false,
        }
    }

    fn next(&self) -> (i16, i16) {
        (self.x + self.dx, self.y + self.dy)
    }

    fn turn(&mut self) {
        // https://wjholden.com/advent-of-code-2017-day22-part1.pdf
        // Switch negative signs to invert (-y is up, +y is down).
        (self.dx, self.dy) = (-self.dy, self.dx);
        assert!(
            (self.dx, self.dy) == (0, 1)
                || (self.dx, self.dy) == (0, -1)
                || (self.dx, self.dy) == (1, 0)
                || (self.dx, self.dy) == (-1, 0)
        );
    }
}

#[allow(dead_code)]
fn show(world: &World, guard: &Guard, max_x: i16, max_y: i16) -> String {
    let mut s = String::new();
    (0..=max_y).for_each(|y| {
        (0..=max_x).for_each(|x| {
            if guard.x == x && guard.y == y {
                s.push(match (guard.dx, guard.dy) {
                    (0, -1) => '^',
                    (1, 0) => '>',
                    (0, 1) => 'v',
                    (-1, 0) => '<',
                    _ => panic!("unexpected guard direction"),
                });
            } else {
                s.push(match world.get(&(x, y)).expect("map element in world") {
                    MapElement::Empty => '.',
                    MapElement::Obstruction => '#',
                });
            }
        });
        s.push('\n');
    });
    s
}

fn main() {
    let puzzle = fs::read_to_string("puzzles/day06.txt").unwrap();
    let result = solve(&puzzle);
    println!("Part 1: {}", result.0);
    println!("Part 2: {}", result.1);
}

fn solve(input: &str) -> (usize, usize) {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut guard = Guard {
        x: 0,
        y: 0,
        dx: 0,
        dy: -1,
    };
    let mut world = World::new();
    input
        .trim()
        .split_whitespace()
        .enumerate()
        .for_each(|(y, line)| {
            line.char_indices().for_each(|(x, c)| {
                max_x = max_x.max(x as i16);
                max_y = max_y.max(y as i16);
                world.insert(
                    (x as i16, y as i16),
                    match c {
                        '.' => MapElement::Empty,
                        '#' => MapElement::Obstruction,
                        '^' => {
                            guard.x = x as i16;
                            guard.y = y as i16;
                            MapElement::Empty
                        }
                        _ => unreachable!("the world should only contain symbols: .#^"),
                    },
                );
            });
        });

    let mut visited: HashSet<(i16, i16)> = HashSet::new();
    let mut part2: HashSet<(i16, i16)> = HashSet::new();
    let guard_initial = (guard.x, guard.y);
    visited.insert(guard_initial);

    loop {
        // Part 2: try finding a cycle
        let next = guard.next();
        if next != guard_initial && !part2.contains(&next) {
            match world.get(&next) {
                Some(MapElement::Empty) => {
                    world.insert(next, MapElement::Obstruction);
                    if is_cyclic_ttl(&world, guard_initial.0, guard_initial.1) {
                        part2.insert(next);
                    }
                    world.insert(next, MapElement::Empty);
                }
                _ => (),
            }
        }

        // Part 1
        if guard.go(&world) {
            visited.insert((guard.x, guard.y));
        } else {
            break;
        }
    }

    (visited.len(), part2.len())
}

#[allow(dead_code)]
fn is_cyclic(world: &World, xi: i16, yi: i16) -> bool {
    let mut guard = Guard {
        x: xi,
        y: yi,
        dx: 0,
        dy: -1,
    };
    let mut path: HashSet<Guard> = HashSet::new();
    loop {
        if path.insert(guard.clone()) == false {
            return true;
        }
        if guard.go(world) == false {
            return false;
        }
    }
}

// You wouldn't think this would be faster, but you save so much copying and hashing
// that just letting the CPU go brr is better.
fn is_cyclic_ttl(world: &World, xi: i16, yi: i16) -> bool {
    let mut guard = Guard {
        x: xi,
        y: yi,
        dx: 0,
        dy: -1,
    };
    let mut steps = 0;
    while steps < world.len() {
        if guard.go(world) == false {
            return false;
        }
        steps += 1;
    }
    true
}

#[cfg(test)]
mod day06 {
    use std::assert_eq;

    use super::*;

    const SAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    // https://www.reddit.com/r/adventofcode/comments/1h81nc0/comment/m0ppjcy/
    const S2: &str = "..........
....#.....
........#.
..........
....^.....
...#......
....#.....
..........
...#......
..........
..#.......
......##..
..........";

    #[test]
    fn test1() {
        assert_eq!(solve(SAMPLE).0, 41)
    }

    #[test]
    fn test2() {
        assert_eq!(solve(SAMPLE).1, 6)
    }

    #[test]
    fn test3() {
        assert_eq!(solve(S2).1, 3)
    }
}

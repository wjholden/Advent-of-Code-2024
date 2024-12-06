use std::{collections::{HashMap, HashSet}, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Guard {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

trait Go {
    fn go(&mut self, world: &World) -> bool;
    fn next(&self, world: &World) -> Option<(i32,i32)>;
}

enum MapElement {
    Empty,
    Obstruction
}

type World = HashMap<(i32,i32), MapElement>;

impl Go for Guard {
    fn go(&mut self, world: &World) -> bool {
        match world.get(&(self.x + self.dx, self.y + self.dy)) {
            Some(MapElement::Empty) => {
                self.x += self.dx;
                self.y += self.dy;
                true
            },
            Some(MapElement::Obstruction) => {
                // https://wjholden.com/advent-of-code-2017-day22-part1.pdf
                // Switch negative signs to change direction.
                (self.dx, self.dy) = (-self.dy, self.dx);
                assert!((self.dx,self.dy) == (0,1) ||
                    (self.dx,self.dy) == (0,-1) ||
                    (self.dx,self.dy) == (1,0) ||
                    (self.dx,self.dy) == (-1,0));
                self.x += self.dx;
                self.y += self.dy;
                true
            }
            None => false
        }
    }

    fn next(&self, world: &World) -> Option<(i32,i32)> {
        match world.get(&(self.x + self.dx, self.y + self.dy)) {
            Some(MapElement::Empty) => Some((self.x + self.dx, self.y + self.dy)),
            Some(MapElement::Obstruction) => Some((self.x - self.dy, self.y + self.dx)),
            None => None
        }
    }
}

fn show(world: &World, guard: &Guard, max_x: i32, max_y: i32) -> String {
    let mut s = String::new();
    (0..max_y).for_each(|y| {
        (0..max_x).for_each(|x| {
            if guard.x == x && guard.y == y {
                s.push(match (guard.dx, guard.dy) {
                    (0,1) => 'v',
                    (0,-1) => '^',
                    (1,0) => '>',
                    (-1,0) => '<',
                    _ => panic!("unexpected guard direction")
                });
            } else {
                s.push(match world.get(&(x,y)).expect("map element in world") {
                    MapElement::Empty => '.',
                    MapElement::Obstruction => '#'
                });
            }
        });    
        s.push('\n');
    });
    s
}

fn main() {
    let puzzle = fs::read_to_string("puzzles/day06.txt").unwrap();
    //println!("{}", puzzle);
    let result = part1(&puzzle);
    println!("Part 1: {}", result.0);
    println!("Part 2: {}", result.1); // 2226 too high
}

fn part1(input: &str) -> (usize, usize) {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut guard = Guard{ x: 0, y: 0, dx: 0, dy: -1 };
    let mut world = World::new();
    input.trim().split_whitespace().enumerate().for_each(|(y, line)| {
        line.char_indices().for_each(|(x,c)| {
            max_x = max_x.max(x);
            max_y = max_y.max(y);
            world.insert((x as i32, y as i32), match c {
                '.' => MapElement::Empty,
                '#' => MapElement::Obstruction,
                '^' => {
                    guard.x = x as i32;
                    guard.y = y as i32;
                    MapElement::Empty
                },
                _ => panic!("unexpected input")
            });
        });
    });

    let mut visited: HashSet<(i32,i32)> = HashSet::new();
    let mut part2 = 0;
    //println!("{}", show(&world, &guard, max_x as i32, max_y as i32));
    loop {
        //println!("Tick. Guard: {:?}", guard);
        //println!("{}", show(&world, &guard, max_x as i32, max_y as i32));

        // Part 2: try finding a cycle
        if let Some(next) = guard.next(&world) {
            match world.get(&next) {
                Some(MapElement::Empty) => {
                    world.insert(next, MapElement::Obstruction);
                    if is_cyclic(&world, &guard) {
                        part2 += 1;
                    }
                    world.insert(next, MapElement::Empty);
                },
                Some(MapElement::Obstruction) => println!("yes, this happens (it never happens)"),
                None => println!("this also happens (it doesn't actually happen)"),
            }
        }
        
        // Part 1
        if guard.go(&world) {
            visited.insert((guard.x, guard.y));
        } else {
            break
        }
    }
    (visited.len(), part2)
}

fn is_cyclic(world: &World, guard: &Guard) -> bool {
    let mut g2 = guard.clone();
    let mut path: HashSet<Guard> = HashSet::new();

    loop {
        if path.contains(&g2) {
            return true
        }
        path.insert(g2.clone());
        if g2.go(world) == false {
            return false
        }
    }
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

    #[test]
    fn test1() {
        assert_eq!(part1(SAMPLE).0, 41)
    }
 
    #[test]
    fn test2() {
        assert_eq!(part1(SAMPLE).1, 6)
    }   
}
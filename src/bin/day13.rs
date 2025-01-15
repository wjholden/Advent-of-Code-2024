use nalgebra::{matrix, Vector2};
use regex::Regex;

fn main() {
    let puzzle = include_str!("../../puzzles/day13.txt");
    println!("Part 1: {}", part1(puzzle));
    println!("Part 2: {}", part2(puzzle, true));
    // 159255020000000 too high
    // 106320600510082.83 too high
    // 102718967795500
}

fn part1(input: &str) -> f64 {
    parse(input).into_iter().filter_map(|machine| machine.tokens()).sum()
}

fn part2(input: &str, shift: bool) -> f64 {
    parse(input).into_iter().map(|mut machine| {
        if shift {
            machine.prize.x += 10000000000000.0;
            machine.prize.y += 10000000000000.0;
        }
        machine
    }).filter_map(|machine| machine.tokens2()).sum()
}

#[derive(Debug)]
struct Machine {
    a: Vector2<f64>,
    b: Vector2<f64>,
    prize: Vector2<f64>
}

impl Machine {
    fn tokens(&self) -> Option<f64> {
        for i in 0..=100 {
            let i = i as f64;
            let ai = self.a * i;
            // Comparison doesn't quite do what you expect here.
            // self.prize < a * i apparently doesn't iterate over each dimension?
            if self.prize.x < ai.x || self.prize.y < ai.y {
                break
            }
            let p = self.prize - ai;
            if p.x / self.b.x == p.y / self.b.y {
                let j = p.x / self.b.x;
                return Some(3.0 * i + j)
            }
        }
        None
    }

    fn tokens2(&self) -> Option<f64> {
        let m = matrix![self.a.x, self.b.x; self.a.y, self.b.y];
        if let Some(inv) = m.try_inverse() {
            let x = inv * self.prize;
            // Yeah, it's ugly. We only want the integer solutions, but with
            // floating-point arithmetic being what it is, we often end up with
            // a fractional part in our result. This looks really sketchy, but
            // it miraculously works out in the end.
            if (0.001 < x.x.fract() && x.x.fract() < 0.999) || (0.001 < x.y.fract() && x.y.fract() < 0.999) {
                None // not an integer solution
            } else {
                Some(3.0 * x.x + x.y)
            }
        } else {
            unreachable!() // this apparently never happens in our problem today
        }
    }
}

fn parse(input: &str) -> Vec<Machine> {
    // Of course there's an unnecessary + in there to confuse the matches...
    let re = Regex::new(r"Button A: X\+(?P<ax>\d+), Y\+(?P<ay>\d+)
Button B: X\+(?P<bx>\d+), Y\+(?P<by>\d+)
Prize: X=(?P<px>\d+), Y=(?P<py>\d+)").unwrap();
    let mut machines = vec![];
    for captures in re.captures_iter(input) {

        let ax = captures.name("ax").unwrap().as_str().parse::<f64>().unwrap();
        let ay = captures.name("ay").unwrap().as_str().parse::<f64>().unwrap();
        let a = Vector2::new(ax, ay);

        let bx = captures.name("bx").unwrap().as_str().parse::<f64>().unwrap();
        let by = captures.name("by").unwrap().as_str().parse::<f64>().unwrap();
        let b = Vector2::new(bx, by);

        let px = captures.name("px").unwrap().as_str().parse::<f64>().unwrap();
        let py = captures.name("py").unwrap().as_str().parse::<f64>().unwrap();
        let prize = Vector2::new(px, py);

        machines.push(Machine { a, b, prize });
    }
    
    machines
}

#[cfg(test)]
mod day13 {
    use std::assert_eq;

    use super::*;

    const SAMPLE: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    const SAMPLE2: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=10000000008400, Y=10000000005400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=10000000012748, Y=10000000012176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=10000000007870, Y=10000000006450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=10000000018641, Y=10000000010279";

    #[test]
    fn test1() {
        assert_eq!(part1(SAMPLE), 480.0)
    }
 
    #[test]
    fn test2() {
        assert_eq!(part1(SAMPLE), part2(SAMPLE, false))
    }  

    #[test]
    fn test3() {
        let mut machines = parse(SAMPLE2).into_iter();
        assert_eq!(None, machines.next().unwrap().tokens2());
        assert!(machines.next().unwrap().tokens2().is_some());
        assert_eq!(None, machines.next().unwrap().tokens2());
        assert!(machines.next().unwrap().tokens2().is_some());
    } 
}
use itertools::Itertools;

fn main() {
    let puzzle = include_str!("../../puzzles/day17.txt");
    println!("Part 1: {}", part1(puzzle));
    println!("Part 2: {}", part2(puzzle));
}

fn part1(input: &str) -> String {
    let (mut computer, program) = parse(input);
    computer.run(&program)
}

fn part2(input: &str) -> usize {
    let (mut computer, program) = parse(input);
    let f = |a: usize| {
        //let a = 8 * a + i;
        let b = a % 8;
        let b = b ^ 3;
        let c = a / (1 << b);
        let b = b ^ 5;
        let _a = a / (1 << 3);
        let b = b ^ c;
        b % 8
    };
    let a = find_a(0, &program, f).unwrap();
    computer.a = a;
    // verify solution
    assert_eq!(computer.run(&program), program.iter().join(","));
    a
}

fn find_a(a: usize, program: &[usize], f: fn(usize) -> usize) -> Option<usize> {
    for i in 0..8 {     
        let a = a * 8 + i;
        let out = f(a);
        if *program.last().unwrap() == out {
            if program.len() == 1 { // base case
                return Some(a)
            } else { // recursive case
                if let Some(a) = find_a(a, &program[..program.len()-1], f) {
                    return Some(a)
                }
            }
        }
    }
    None
}

fn parse(input: &str) -> (Computer, Vec<usize>) {
    let mut lines = input.lines();
    let a = lines.next().unwrap().split_whitespace().last().unwrap().parse::<usize>().unwrap();
    let program = lines.last().unwrap()[9..].split(",").map(|s| s.parse::<usize>().unwrap()).collect();
    let computer = Computer {
        ip: 0,
        a,
        b: 0,
        c: 0
    };
    (computer,  program)
}

#[derive(Copy, Clone)]
pub struct Computer {
    ip: usize,
    a: usize,
    b: usize,
    c: usize,
}

impl Computer {
    pub fn run(&mut self, program: &[usize]) -> String {
        let _instruction_names = ["adv", "bxl", "bst", "jnz",
            "bxc", "out", "bdv", "cdv"];

        let mut output: Vec<usize> = vec![];

        loop {
            if self.ip >= program.len() {
                break
            }

            let opcode = program[self.ip];
            let operand = program[self.ip + 1];
            assert!(opcode < 8);

            match opcode {
                0 => self.adv(operand),
                1 => self.bxl(operand),
                2 => self.bst(operand),
                3 => self.jnz(operand),
                4 => self.bxc(),
                5 => output.push(self.out(operand)),
                6 => self.bvd(operand),
                7 => self.cdv(operand),
                _ => unreachable!(),
            };

            // except for jumps, increment instruction pointer by 2.
            if opcode != 3 {
                self.ip += 2;
            }
        }

        output.into_iter().join(",")
    }

    fn combo_argument(&self, operand: usize) -> usize {
        match operand {
            literal @ 0..=3 => literal,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            other => panic!("illegal operand {other}"),
        }
    }

    fn adv(&mut self, operand: usize) {
        self.a /= 1 << self.combo_argument(operand)
    }

    fn bxl(&mut self, operand: usize) {
        self.b ^= operand
    }

    fn bst(&mut self, operand: usize) {
        self.b = self.combo_argument(operand) % 8
    }

    fn jnz(&mut self, operand: usize) {
        if self.a == 0 {
            // The jnz instruction (opcode 3) does nothing if the A register is 0.
            self.ip += 2
        } else {
            // However, if the A register is not zero, it jumps by setting the
            // instruction pointer to the value of its literal operand
            self.ip = operand
        }
    }

    fn bxc(&mut self) {
        self.b ^= self.c
    }

    fn out(&mut self, operand: usize) -> usize {
        self.combo_argument(operand) % 8
    }

    fn bvd(&mut self, operand: usize) {
        self.b = self.a / (1 << self.combo_argument(operand))
    }

    fn cdv(&mut self, operand: usize) {
        self.c = self.a / (1 << self.combo_argument(operand))
    }
}

#[cfg(test)]
mod day17 {
    use std::assert_eq;

    use super::*;

    const SAMPLE: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const QUINE: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn test1() {
        assert_eq!(part1(SAMPLE), "4,6,3,5,6,3,5,2,1,0")
    }

    #[test]
    fn test2() {
        let (_, program) = parse(QUINE);
        let g = |a| {
            let a = a / (1 << 3);
            a % 8
        };
        assert_eq!(find_a(0, &program, g).unwrap(), 117440)
    }

    #[test]
    fn quine() {
        let (mut computer, program) = parse(QUINE);
        computer.a = 117440;
        assert_eq!(computer.run(&program), "0,3,5,4,3,0")
    }

    #[test]
    fn parser() {
        let (computer, program) = parse(SAMPLE);
        assert_eq!(computer.a, 729);
        assert_eq!(computer.b, 0);
        assert_eq!(computer.c, 0);
        assert_eq!(computer.ip, 0);
        assert_eq!(program, vec![0,1,5,4,3,0]);
    }
}

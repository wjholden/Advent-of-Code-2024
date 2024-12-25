use std::collections::BTreeMap;

use itertools::Itertools;
use pathfinding::prelude::astar;

fn main() {
    let puzzle = include_str!("../../puzzles/day24.txt");
    println!("Part 1: {}", part1(&puzzle));
    //println!("Part 2: {}", part2(&puzzle));
}

fn part1(input: &str) -> u64 {
    let (mut literals, gates) = parse(input);

    let mut result = 0;
    for literal in gates.keys().rev() {
        if literal.starts_with("z") {
            let value = eval(literal, &mut literals, &gates) as u64;
            println!("{literal} = {value}");
            result = (result << 1) | value;
        }
    }
    result
}

fn part2(input: &str) -> String {
    let mut adder = Adder::new(input);
    adder.set_one_zero();
    let adder = adder;

    // There are about 24000 pairs of possible swaps.
    // We can probably reduce this (a lot) by looking only at the "dirty"
    // gates that didn't come up with the expected answer for z.
    let n = adder.pairs.len();
    println!("There are {} pairs. First 5: {:?}.", n, &adder.pairs[..5]);

    let start = (0_usize, n / 3, 2 * n / 3, n-1);

    let successors = |t: &(usize, usize, usize, usize)| {
        //println!("visit {t:?}");
        let (x,y,z,w) = *t;
        vec![
            (x + 1, y, z, w),
            (x + 1000, y, z, w),
            //(x-1,y,z,w), // Start at x=0, so never decrease x.
            (x, y + 1, z, w),
            (x, y - 1, z, w),
            (x, y - 1000, z, w),
            (x, y, z + 1000, w),
            (x, y, z + 1, w),
            (x, y, z - 1, w),
            //(x, y, z, w + 1), // Start at w=n, so never increase n.
            (x, y, z, w - 1000),
            (x, y, z, w - 1),
        ]
        .into_iter()
        .filter_map(|(x, y, z, w)| {
            if x < y && y < z && z < w {
                Some(((x, y, z, w), 10))
            } else {
                None
            }
        })
        .collect_vec()
    };

    let heuristic = |t: &(usize,usize,usize,usize)| {
        let (x,y,z,w) = *t;
        match adder.swap(&(x,y,z,w)) {
            Some(candidate) => candidate.error(),
            None => u64::MAX
        }
    };

    let success = |t: &(usize,usize,usize,usize)| {
        //println!("success? {t:?}");
        let (x,y,z,w) = *t;
        match adder.swap(&(x,y,z,w)) {
            Some(candidate) => {
                println!("success? {t:?} err = {}", candidate.error());
                candidate.error() == 0
            },
            None => false
        }
    };

    //if let Some((path, _)) = astar(&start, successors, heuristic, success) {
    //    println!("Found a solution: {path:?}");
    //}

    if let Some((path, dist)) = astar(&start, successors, heuristic, success) {
        println!("{path:?}, {dist}");
    } else {
        println!("no path?");
    }

    String::from("test")
}

fn _part2(input: &str) -> String {
    let (mut literals, gates) = parse(input);

    for (label, literal) in literals.iter_mut() {
        if label.starts_with("x") {
            *literal = 1;
        } else {
            *literal = 0;
        }
    }
    println!("{literals:?}");

    for literal in gates.keys().rev() {
        if literal.starts_with("z") {
            let value = eval(literal, &mut literals, &gates);
            if value != 1 {
                investigate(&literal, 1, &literals, &gates);
            }
        }
    }

    String::from("test")
}

fn investigate(x: &str, expect: u8, literals: &BTreeMap<&str, u8>, gates: &BTreeMap<&str, Gate>) {
    let got = literals.get(x).unwrap();
    if !gates.contains_key(x) {
        println!("({x}: expected {expect}, got {got} from x/y literal.");
        return;
    }
    let gate = gates.get(x).unwrap();
    let left = gate.left.as_str();
    let right = gate.right.as_str();
    println!(
        "{x}: expected {expect}, got {got} (left = {}, right = {})",
        gate.left, gate.right
    );
    match &gate.instruction {
        Instruction::And => {
            // easy case
            println!("This was an AND gate. We can tell which one is wrong.");
            let lvalue = literals.get(left).unwrap();
            if *lvalue != 1 {
                investigate(left, 1, literals, gates);
            }
            let rvalue = literals.get(right).unwrap();
            if *rvalue != 1 {
                investigate(right, 1, literals, gates);
            }
        }
        Instruction::Or => {
            println!("This was an OR gate.");
        }
        Instruction::Xor => {
            println!("This was an XOR gate.");
        }
    }
}

fn eval<'a>(
    x: &'a str,
    literals: &mut BTreeMap<&'a str, u8>,
    gates: &'a BTreeMap<&str, Gate>,
) -> u8 {
    if let Some(value) = literals.get(x) {
        return *value;
    }

    let gate = gates.get(x).unwrap();
    let left = eval(&gate.left, literals, gates);
    let inst = &gate.instruction;
    let right = eval(&gate.right, literals, gates);
    let result = match inst {
        Instruction::And => left & right,
        Instruction::Or => left | right,
        Instruction::Xor => left ^ right,
    };

    literals.insert(x, result);
    result
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone)]
struct Gate {
    left: String,
    instruction: Instruction,
    right: String,
}

#[derive(Clone)]
struct Adder {
    literals: BTreeMap<String, u8>,
    gates: BTreeMap<String, Gate>,
    pairs: Vec<(String, String)>,
}

impl Adder {
    fn new(input: &str) -> Adder {
        let mut s = input.split("\n\n");
        let s1 = s.next().unwrap();
        let s2 = s.next().unwrap();
        let mut literals = BTreeMap::new();
        let mut gates = BTreeMap::new();

        for line in s1.lines() {
            let label = &line[..3];
            let value = line[5..].parse::<u8>().unwrap();
            literals.insert(String::from(label), value);
        }

        for line in s2.lines() {
            let mut tokens = line.split_whitespace();
            let left = tokens.next().unwrap();
            let inst = match tokens.next().unwrap() {
                "AND" => Instruction::And,
                "OR" => Instruction::Or,
                "XOR" => Instruction::Xor,
                _ => unreachable!(),
            };
            let right = tokens.next().unwrap();
            _ = tokens.next();
            let result = tokens.next().unwrap();
            gates.insert(
                String::from(result),
                Gate {
                    left: String::from(left),
                    instruction: inst,
                    right: String::from(right),
                },
            );
        }

        let pairs = gates
            .keys()
            .combinations(2)
            .map(|v| (v[0].clone(), v[1].clone()))
            .collect_vec();

        Adder {
            literals,
            gates,
            pairs,
        }
    }

    fn eval(&self, x: &str, limit: u8) -> Result<u8, ()> {
        if limit == 0 {
            return Err(()) // overflow
        }

        //println!("I need {x}");
        if let Some(value) = self.literals.get(x) {
            return Ok(*value);
        }

        let gate = self.gates.get(x).unwrap();
        let left = self.eval(&gate.left, limit - 1);
        let inst = &gate.instruction;
        let right = self.eval(&gate.right, limit - 1);

        if left.is_err() || right.is_err() {
            return Err(())
        }

        let left = left.unwrap();
        let right = right.unwrap();

        let result = match inst {
            Instruction::And => left & right,
            Instruction::Or => left | right,
            Instruction::Xor => left ^ right,
        };

        Ok(result)
    }

    fn set_one_zero(&mut self) {
        // Set all the x inputs to 1 and the y inputs to 0.
        // The resulting z values should all be 1.
        for (label, literal) in self.literals.iter_mut() {
            if label.starts_with("x") {
                *literal = 1;
            } else {
                *literal = 0;
            }
        }
    }

    fn swap(&self, t: &(usize, usize, usize, usize)) -> Option<Adder> {
        // First, verify that all the swaps are distinct.
        let (x, y, z, w) = *t;
        let distinct = [
            &self.pairs[x].0,
            &self.pairs[x].1,
            &self.pairs[y].0,
            &self.pairs[y].1,
            &self.pairs[z].0,
            &self.pairs[z].1,
            &self.pairs[w].0,
            &self.pairs[w].1,
        ]
        .into_iter()
        .all_unique();
        if !distinct {
            return None
        }

        // Ok, our swaps are feasible so let's do this.
        let mut gates = self.gates.clone();
        for i in [x, y, z, w] {
            let (a, b) = &self.pairs[i];
            let ga = gates.get(a).unwrap().clone();
            let gb = gates.get(b).unwrap().clone();
            gates.insert(a.to_string(), gb);
            gates.insert(b.to_string(), ga);
        }

        Some(Adder{
            literals: self.literals.clone(),
            gates,
            pairs: self.pairs.clone(),
        })
    }

    fn error(&self) -> u64 {
        let mut h = 0;
        for literal in self.gates.keys().rev() {
            if literal.starts_with("z") {
                let got = self.eval(&literal, 48);
                //println!("{literal}: {got:?}");
                if got != Ok(1) {
                    h += 1
                }
            }
        }
        h
    }
}

fn parse(input: &str) -> (BTreeMap<&str, u8>, BTreeMap<&str, Gate>) {
    let mut s = input.split("\n\n");
    let s1 = s.next().unwrap();
    let s2 = s.next().unwrap();
    let mut literals = BTreeMap::new();
    let mut gates = BTreeMap::new();

    for line in s1.lines() {
        let label = &line[..3];
        let value = line[5..].parse::<u8>().unwrap();
        literals.insert(label, value);
    }

    for line in s2.lines() {
        let mut tokens = line.split_whitespace();
        let left = tokens.next().unwrap();
        let inst = match tokens.next().unwrap() {
            "AND" => Instruction::And,
            "OR" => Instruction::Or,
            "XOR" => Instruction::Xor,
            _ => unreachable!(),
        };
        let right = tokens.next().unwrap();
        _ = tokens.next();
        let result = tokens.next().unwrap();
        gates.insert(
            result,
            Gate {
                left: String::from(left),
                instruction: inst,
                right: String::from(right),
            },
        );
    }

    (literals, gates)
}

#[cfg(test)]
mod day24 {
    use std::assert_eq;

    use super::*;

    const SMALL: &str = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

    const SAMPLE: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    const SAMPLE2: &str = "x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00";

    #[test]
    fn small() {
        assert_eq!(part1(SMALL), 4)
    }

    #[test]
    fn test1() {
        assert_eq!(part1(SAMPLE), 2024)
    }

    #[test]
    fn test2() {
        part1(SAMPLE2);
        //assert_eq!(part2(SAMPLE), 0)
    }
}

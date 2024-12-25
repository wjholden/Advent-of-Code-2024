use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;
use nalgebra::*;

fn main() {
    let puzzle = include_str!("../../puzzles/day23.txt");
    println!("Part 1: {}", part1_naive(&puzzle));
    //println!("Part 1: {}", part1_matrix(&puzzle));
    println!("Part 2: {}", part2(&puzzle));
}

fn parse(input: &str) -> BTreeMap<&str, BTreeSet<&str>> {
    let mut edges: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for line in input.lines() {
        let u = &line[..2];
        let v = &line[3..];
        edges.entry(u).or_insert(BTreeSet::new()).insert(v);
        edges.entry(v).or_insert(BTreeSet::new()).insert(u);
    }
    edges
}

// It would have been really cool to use matrix multiplication for this.
// Let A be an adjacency matrix for graph G.
// Then for all positions where (A .* A^2) are nonzero, there is a triangle.
fn part1_naive(input: &str) -> usize {
    let edges = parse(input);

    let mut count = 0;
    // Intentional cubic-time algorithm, calm down.
    for a in edges.keys() {
        for (b, eb) in edges.iter() {
            if a >= b || !eb.contains(a) {
                continue
            }
            for (c, ec) in edges.iter() {
                if a >= c || b >= c || !ec.contains(a) || !ec.contains(b) {
                    continue
                }
                if a.starts_with("t") || 
                    b.starts_with("t") ||
                    c.starts_with("t") { // found a triangle
                    //println!("{a},{b},{c}");
                    count += 1;
                }
            }
        }
    }
    count
}

#[allow(dead_code)]
// https://stackoverflow.com/questions/10193228/how-to-find-a-triangle-inside-a-graph
// https://i11www.iti.kit.edu/extra/publications/sw-fclt-05_wea.pdf
fn part1_matrix(input: &str) -> usize {
    let edges = parse(input);
    let labels = edges.keys().cloned().collect_vec();
    //println!("{edges:?}");
    //println!("{labels:?}");
    
    let n = labels.len();
    
    let m1 = DMatrix::from_fn(n, n, |r,c| {
        let u = labels[r];
        let v = labels[c];
        if edges.get(u).unwrap().contains(v) {
            1
        } else {
            0
        }
    });

    // https://www.nalgebra.org/docs/user_guide/vectors_and_matrices/
    let m2 = m1.clone() * m1.clone();
    let m3 = m2.component_mul(&m1);

    //println!("m1={m1}m2={m2}m3={m3}");
    //println!("{labels:?}");

    let mut total = 0;

    for (row, label) in labels.iter().enumerate() {
        if label.starts_with("t") {
            let m4 = m3.row(row);
            for (col, label) in labels.iter().enumerate() {
                if !label.starts_with("t") {
                    total += m4[col];
                }
            }
        }
    }

    total/2
}

fn part2(input: &str) -> String {
    let g = parse(input);
    
    let mut longest = BTreeSet::new();

    for (v, neighbors) in g.iter() {
        if v.starts_with("t") {
            let mut clique = BTreeSet::new();
            clique.insert(*v);
            clique = max_clique(&g, clique, neighbors.clone());
            if longest.len() < clique.len() {
                longest = clique;
            }
        }
    }

    longest.iter().join(",")
}

fn max_clique<'a>(g: &BTreeMap<&'a str, BTreeSet<&'a str>>, clique: BTreeSet<&'a str>, mut candidates: BTreeSet<&'a str>) -> BTreeSet<&'a str> {
    if candidates.is_empty() {
        return clique
    }

    let mut result = BTreeSet::new();

    loop {
        if let Some(candidate) = candidates.pop_first() {
            let neighbors = g.get(candidate).unwrap();
        if clique.is_subset(neighbors) {
            let mut bigger_clique = clique.clone();
            bigger_clique.insert(candidate);
            let smaller_candidates = candidates.intersection(neighbors).copied().collect::<BTreeSet<&str>>();
            let c = max_clique(g, bigger_clique, smaller_candidates);
            if result.len() < c.len() {
                result = c;
            }
        }
        } else {
            break
        }
    }

    return result
}

#[cfg(test)]
mod day23 {
    use std::assert_eq;

    use super::*;

    const SAMPLE: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn naive() {
        assert_eq!(part1_naive(SAMPLE), 7)
    }

    #[test]
    fn part1() {
        assert_eq!(part1_matrix(SAMPLE), 7)
    }

    #[test]
    fn tiny() {
        let small = "aa-bb\nbb-cc\naa-cc\ncc-dd\nbb-dd";
        part1_matrix(&small);
    }

    #[test]
    fn kite() {
        let x = "aa-bb
aa-cc
aa-dd
bb-cc
bb-dd
cc-dd
cc-ee
ee-ff
ee-gg
ee-th
ff-gg
ff-th
gg-th
th-ii
gg-ii";
        assert_eq!(part1_matrix(x), part1_naive(x))
    }
 
    #[test]
    fn test2() {
        assert_eq!(part2(SAMPLE), "co,de,ka,ta")
    }   
}
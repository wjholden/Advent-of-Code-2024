use std::collections::BTreeMap;

use itertools::Itertools;

fn main() {
    let puzzle = include_str!("../../puzzles/day19.txt");
    let (part1, part2) = solve(&puzzle);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn solve(input: &str) -> (usize, usize) {
    let (root, lines) = parse(input);
    
    let mut part1 = 0;
    let mut part2 = 0;
    for line in lines {
        let mut cache = BTreeMap::new();
        let x = search(&root, &line.chars().collect_vec(), &mut cache);
        if x > 0 {
            part1 += 1;
            part2 += x;
        }
    }

    (part1, part2)
}


fn search<'a>(trie: &TrieNode, chars: &'a [char], cache: &mut BTreeMap<&'a [char], usize>) -> usize {
    if cache.contains_key(chars) {
        return *cache.get(chars).unwrap();
    }

    if chars.len() == 0 {
        return 1
    }
    let lengths = trie.matches(chars);
    let mut solution = 0;
    for matched_length in lengths.iter() {
        let chars = &chars[*matched_length..];
        let x = search(trie, chars, cache);
        if x > 0 {
            solution = solution + x;
        }
    }

    cache.insert(chars, solution);
    solution
}

struct TrieNode {
    children: BTreeMap<char,TrieNode>,
    terminus: Option<()>
}

impl TrieNode {
    // This is only for the root node.
    fn new() -> TrieNode {
        TrieNode {
            children: BTreeMap::new(),
            terminus: None
        }
    }

    fn insert(&mut self, chars: &[char]) {
        if chars.is_empty() {
            self.terminus = Some(());
            return
        }

        let c = chars[0];
        let child = self.children.entry(c).or_insert(TrieNode::new());
        child.insert(&chars[1..]);
    }

    fn matches(&self, chars: &[char]) -> Vec<usize> {
        if chars.is_empty() {
            if self.terminus.is_some() {
                return vec![0]
            } else {
                return vec![]
            }
        }

        let c = chars[0];
        if let Some(child) = self.children.get(&c) {
            let mut m = child.matches(&chars[1..]);
            for depth in m.iter_mut() {
                *depth += 1;
            }
            if child.terminus.is_some() && chars.len() > 1 {
                m.push(1);
            }
            m
        } else {
            vec![]
        }
    }
}

fn parse(input: &str) -> (TrieNode, Vec<&str>) {
    let mut input = input.split("\n\n");
    let patterns = input.next().unwrap();
    let designs = input.next().unwrap();
    let mut root = TrieNode::new();
    for pattern in patterns.split(", ") {
        let chars = pattern.chars().collect_vec();
        root.insert(&chars);
    }
    (root, designs.lines().collect())
}

#[cfg(test)]
mod day19 {
    use std::assert_eq;

    use super::*;

    const SAMPLE: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test1() {
        assert_eq!(solve(SAMPLE).0, 6)
    }
 
    #[test]
    fn test2() {
        assert_eq!(solve(SAMPLE).1, 16)
    }   
}

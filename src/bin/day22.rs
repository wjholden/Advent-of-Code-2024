use std::collections::HashMap;

fn main() {
    let puzzle = include_str!("../../puzzles/day22.txt");
    println!("Part 1: {}", part1(&puzzle));
    println!("Part 2: {}", part2(&puzzle));
}

fn part1(input: &str) -> i64 {
    input.trim().lines().map(|line| {
        let mut i = line.parse().unwrap();
        for _ in 1..=2000 {
            next(&mut i);
        }
        i
    }).sum()
}

// Reference https://www.reddit.com/r/adventofcode/comments/1hjroap/comment/m390cfi/
// for a very clean and concurrent solution.
fn part2(input: &str) -> i64 {
    let secrets: Vec<i64> = input.trim().lines().map(|line| line.parse().unwrap()).collect();

    let mut sales = HashMap::new();

    let b = secrets.into_iter().map(|secret| {
        let (prices, changes) = seq(secret, 2000);
        let mut bananas = HashMap::new();
        let n = prices.len();
        for i in 3..n {
            if let [a,b,c,d] = changes[i-3..=i] {
                let sequence = (a,b,c,d);
                let price = prices[i];
                if !bananas.contains_key(&sequence) {
                    bananas.insert(sequence, price);
                }
            }
        }
        bananas
    });

    for bananas in b {
        for (k, v) in bananas {
            *sales.entry(k).or_insert(0) += v;
        }
    }

    sales.into_values().max().unwrap()
}

fn seq(secret: i64, length: usize) -> (Vec<i64>, Vec<i64>) {
    let mut prices = vec![];
    let mut changes = vec![];
    let mut current = secret;
    for _ in 1..=length {
        let previous = current;
        next(&mut current);
        prices.push(current%10);
        changes.push((current%10) - (previous%10));
    }
    (prices, changes)
}

fn next(secret: &mut i64) -> i64 {
    mix(secret, 64 * *secret);
    prune(secret);
    
    mix(secret, *secret / 32);
    prune(secret);
    
    mix(secret, *secret * 2048);
    prune(secret);

    *secret
}

fn mix(secret: &mut i64, x: i64) {
    *secret ^= x
}

fn prune(secret: &mut i64) {
    *secret %= 16777216
}

#[cfg(test)]
mod day21 {
    use std::assert_eq;

    use super::*;

    const SAMPLE: &str = "1
10
100
2024";

    const SAMPLE2: &str ="1
2
3
2024";

    #[test]
    fn test123() {
        let mut secret = 123;
        assert_eq!(next(&mut secret), 15887950);
        assert_eq!(next(&mut secret), 16495136);
        assert_eq!(next(&mut secret), 527345);
        assert_eq!(next(&mut secret), 704524);
        assert_eq!(next(&mut secret), 1553684);
        assert_eq!(next(&mut secret), 12683156);
        assert_eq!(next(&mut secret), 11100544);
        assert_eq!(next(&mut secret), 12249484);
        assert_eq!(next(&mut secret), 7753432);
        assert_eq!(next(&mut secret), 5908254);
    }

    #[test]
    fn sequences() {

    }

    #[test]
    fn test1() {
        assert_eq!(part1(SAMPLE), 37327623)
    }
 
    #[test]
    fn test2() {
        assert_eq!(part2(SAMPLE2), 23)
    }   
}
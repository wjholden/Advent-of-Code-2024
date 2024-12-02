use std::fs;

fn main() {
    let puzzle = fs::read_to_string("day02.txt").unwrap();
    let p = parse(&puzzle.trim());
    println!("Part 1: {}", part1(&p, false));
    println!("Part 2: {}", part1(&p, true));
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    input.split("\n").map(|line| {
        line.split_whitespace().map(|e| {
            e.parse().unwrap()
        }).collect()
    }).collect()
}

fn part1(input: &Vec<Vec<i32>>, dampener: bool) -> i32 {
    input.iter().fold(0, |acc, row| {
        // slope also cannot be zero.
        let slope = row.last().expect("last") - row.first().expect("first");
        let mut faults = 0;
        let mut damper_used = false;
        for (i, &x) in row.iter().enumerate() {
            if i == 0 {
                continue; // skip first
            }
            // So many cases to consider in part 2!
            if is_unsafe(slope, row[i-1], x) {
                faults += 1;
                
                //println!("{:?} is unsafe at {} and {}", row, row[i-1], x);
                if dampener && !damper_used {
                    if i <= row.len() - 2 {
                        if !is_unsafe(slope, row[i-1], row[i+1]) {
                            damper_used = true
                        }
                    } else {
                        damper_used = true
                    }
                }
            }
        }
        
        if faults == 0 {
            acc + 1
        } else if faults == 1 && damper_used {
            acc + 1
        } else {
            acc
        }
    })
}

fn is_unsafe(slope: i32, x1: i32, x2: i32) -> bool {
    let dr = x2 - x1;
    slope * dr <= 0 || dr.abs() < 1 || dr.abs() > 3
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    const SAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test1() {
        let s = parse(SAMPLE);
        assert_eq!(part1(&s, false), 2)
    }
 
    #[test]
    fn test2() {
        let s = parse(SAMPLE);
        assert_eq!(part1(&s, true), 4)
    }   
}
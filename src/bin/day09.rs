use std::{fmt, fs};

fn main() {
    let puzzle = fs::read_to_string("puzzles/day09.txt").unwrap();
    let mut disk = parse(&puzzle);
    println!("Part 1: {}", part1(&mut disk));
    println!("Part 2: {}", part2(&puzzle));
}

#[derive(Debug)]
enum Block {
    Data{
        id: u64
    },
    Empty,
}

type Disk = Vec<Block>;

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Block::Data{id} => write!(f, "{id}"),
            Block::Empty => write!(f, ".")
        }
    }
}

fn parse(input: &str) -> Disk {
    input.trim().char_indices().map(|(index, blocks)| {
        let blocks = blocks.to_digit(10).unwrap();
        (0..blocks).map(move |_| match index % 2 {
            0 => Block::Data{id: index as u64 / 2},
            1 => Block::Empty,
            _ => unreachable!("fault")
        })
    }).flatten().collect()
}

#[allow(dead_code)]
fn print_disk(disk: Disk) {
    for block in disk.iter() {
        print!("{}", block)
    }
    println!()
}

#[allow(dead_code)]
fn empty_block_count(disk: &[Block]) -> usize {
    disk.iter().filter(|block| matches!(block, Block::Empty)).count()
}

fn checksum(disk: &[Block]) -> u64 {
    disk.iter().enumerate().filter_map(|(i,val)| {
        match val {
            Block::Empty => None,
            Block::Data{id} => Some((i as u64) * id)
        }
    }).sum()
}

fn part1(disk: &mut [Block]) -> u64 {
    let mut left = 0;
    let mut right = disk.len()-1;
    loop {
        loop {
            match disk[left] {
                Block::Empty => break,
                Block::Data{ id: _ } => left += 1
            }
        }
        loop {
            match disk[right] {
                Block::Empty => right -= 1,
                Block::Data{ id: _ } => break
            }
        }
        if left == right {
            unreachable!("left and right should never match");
        }
        if right < left {
            break // finished sorting disk
        }
        disk.swap(left, right);
        //print_disk(disk);
    }
    checksum(disk)
}

#[derive(Debug, Clone, Copy)]
struct File {
    id: usize,
    length: usize,
    position: usize
}
#[derive(Debug, Clone, Copy)]
struct Free {
    length: usize,
    position: usize
}

struct Disk2 {
    files: Vec<File>,
    frees: Vec<Free>
}

impl ToString for Disk2 {
    fn to_string(&self) -> String {
        let mut s = String::with_capacity(self.blockcount());
        for file in self.files.iter() {
            for i in file.position..file.position+file.length {
                s.replace_range(i..i+1, &file.id.to_string());
            }
        }
        for free in self.frees.iter() {
            for i in free.position..free.position+free.length {
                s.replace_range(i..i+1,".");
            }
        }
        s
    }
}

impl Disk2 {
    fn new(input: &str) -> Disk2 {
        let mut files = vec![];
        let mut frees = vec![];
        let mut position = 0;
        input.char_indices().for_each(|(index,c)| {
            let length = c.to_digit(10).unwrap() as usize;
            let id = index/2;
            match index%2 {
                0 => files.push(File{id, length, position}),
                1 => frees.push(Free{length, position}),
                _ => unreachable!()
            };
            position += length;
        });
        Disk2{ files, frees }
    }

    fn blockcount(&self) -> usize {
        self.files.iter().map(|file| file.length).sum::<usize>() + self.frees.iter().map(|free| free.length).sum::<usize>()
    }

    fn first_free(&self, file: &File) -> Option<usize> {
        for i in 0..self.frees.len() {
            if self.frees[i].length >= file.length && self.frees[i].position < file.position {
                return Some(i)
            }
        }
        None
    }

    fn defrag(&mut self) {
        for j in (0..self.files.len()).rev() {
            let mut file = self.files[j];
            println!("defrag file {} with length {}", file.id, file.length);
            println!("{}", self.to_string());
            match self.first_free(&file) {
                Some(j) => {
                    let mut free = self.frees[j];
                    (free.position, file.position) = (file.position, free.position);
                    continue
                },
                None => {
                    
                }
            };
            println!("there was nowhere to put file {}", file.id);
        }
    }
}

fn part2(input: &str) -> usize {
    let mut disk = Disk2::new(input);
    println!("I think I have {} blocks, and they look like:", disk.blockcount());
    println!("{}", disk.to_string());

    disk.defrag();

    println!("{}", disk.to_string());
    
    disk.files.into_iter().map(|file| {
        (file.position..file.position+file.length).into_iter().sum::<usize>() * file.id
    }).sum()
}

#[cfg(test)]
mod day09 {
    use std::assert_eq;

    use super::*;

    const SAMPLE: &str = "2333133121414131402";

    #[test]
    fn test1() {
        let mut disk = parse(SAMPLE);
        assert_eq!(part1(&mut disk), 1928)
    }

    #[test]
    fn empties() {
        let disk = parse(SAMPLE);
        assert_eq!(empty_block_count(&disk), "..............".len())
    }
 
    #[test]
    fn test2() {
        assert_eq!(part2(&SAMPLE), 2858)
    }   
}
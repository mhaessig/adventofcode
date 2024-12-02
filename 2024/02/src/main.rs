use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Unknown,
    Increasing,
    Decreasing,
}

struct ReactorReport {
    levels: Vec<u8>,
}

impl ReactorReport {
    pub fn new(levels: Vec<u8>) -> Self {
        Self { levels }
    }

    pub fn is_safe(&self) -> bool {
        let mut dir = Direction::Unknown;
        for i in 1..self.levels.len() {
            let prev = self.levels.get(i - 1).unwrap();
            let this = self.levels.get(i).unwrap();

            let diff = prev.abs_diff(*this);
            if diff == 0 || diff > 3 {
                return false;
            }

            match dir {
                Direction::Unknown => {
                    if prev < this {
                        dir = Direction::Increasing;
                    } else if prev > this {
                        dir = Direction::Decreasing;
                    }
                }
                Direction::Decreasing => {
                    if prev < this {
                        return false;
                    }
                }
                Direction::Increasing => {
                    if prev > this {
                        return false;
                    }
                }
            }
        }

        true
    }

    pub fn safe_with_dampener(&self) -> bool {
        if self.is_safe() {
            return true;
        }

        for i in 0..self.levels.len() {
            let mut smaller = self.levels.clone();
            smaller.remove(i);
            let smaller = ReactorReport::new(smaller);
            if smaller.is_safe() {
                return true;
            }
        }

        false
    }
}

impl FromStr for ReactorReport {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let levels = s
            .split_ascii_whitespace()
            .map(|s| s.parse::<u8>().unwrap())
            .collect();
        Ok(Self { levels })
    }
}

fn solution(r: BufReader<File>) -> Result<(u64, u64), Box<dyn Error>> {
    let mut reports: Vec<ReactorReport> = Vec::new();

    for line in r.lines() {
        let line = line?;

        reports.push(ReactorReport::from_str(&line)?)
    }

    let part1 = reports
        .iter()
        .map(|r| r.is_safe())
        .map(|b| if b { 1 } else { 0 })
        .sum();

    let part2 = reports
        .iter()
        .map(|r| r.safe_with_dampener())
        .map(|b| if b { 1 } else { 0 })
        .sum();

    Ok((part1, part2))
}

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input.txt")?;
    let r = BufReader::new(f);

    let (part1, part2) = solution(r)?;

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

#[test]
fn test() {
    let f = File::open("test_input.txt").unwrap();
    let r = BufReader::new(f);

    assert_eq!(solution(r).unwrap(), (2, 4))
}

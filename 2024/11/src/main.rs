use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Stone(u64);

#[derive(Debug, PartialEq, Eq)]
enum BlinkedStone {
    Single(Stone),
    Double(Stone, Stone),
}

impl Stone {
    #[allow(dead_code)]
    fn new(n: u64) -> Self {
        Self(n)
    }

    fn len(&self) -> u32 {
        if self.0 == 0 {
            1
        } else {
            // Fine to unwrap since only have integers.
            self.0.ilog10() + 1
        }
    }

    fn blink(&self) -> BlinkedStone {
        if self.0 == 0 {
            BlinkedStone::Single(1.into())
        } else if self.len() % 2 == 0 {
            let len_half = self.len() / 2;
            let left = self.0 / 10u64.pow(len_half);
            let right = self.0 - left * 10u64.pow(len_half);
            BlinkedStone::Double(left.into(), right.into())
        } else {
            BlinkedStone::Single((self.0 * 2024).into())
        }
    }

    fn blink_n(&self, n: u8) -> u64 {
        if n == 0 {
            return 1;
        }
        match self.blink() {
            BlinkedStone::Single(st) => st.blink_n(n - 1),
            BlinkedStone::Double(left, right) => left.blink_n(n - 1) + right.blink_n(n - 1),
        }
    }
}

impl From<u64> for Stone {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl FromStr for Stone {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse::<u64>()?.into())
    }
}

fn solution(r: BufReader<File>) -> Result<(u64, u64), Box<dyn Error>> {
    let stones = r
        .lines()
        .map_while(Result::ok)
        .collect::<String>()
        .split_whitespace()
        .map(|s| Stone::from_str(s).unwrap())
        .collect::<Vec<Stone>>();

    let part1 = stones.iter().map(|s| s.blink_n(25)).sum();
    let part2 = 0;

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
fn test_blink() {
    assert_eq!(Stone::new(0).blink(), BlinkedStone::Single(1.into()));
    assert_eq!(
        Stone::new(99).blink(),
        BlinkedStone::Double(9.into(), 9.into())
    );
    assert_eq!(
        Stone::new(1234).blink(),
        BlinkedStone::Double(12.into(), 34.into())
    );
    assert_eq!(Stone::new(1).blink(), BlinkedStone::Single(2024.into()));
}

#[test]
fn test() {
    let f = File::open("test_input.txt").unwrap();
    let r = BufReader::new(f);

    assert_eq!(solution(r).unwrap().0, 55312)
}

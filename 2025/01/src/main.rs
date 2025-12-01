use std::{
    error::Error,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader, Read},
    str::FromStr,
};

use libaoc::*;
use regex::Regex;

#[derive(Clone, Copy, Debug)]
struct Dial(i32);

impl Dial {
    const MIN: i32 = 0;
    const MAX: i32 = 99;
    const MOD: i32 = Self::MAX + 1;

    fn new(val: i32) -> Result<Self, String> {
        if val < Self::MIN && val > Self::MAX {
            return Err(format!("Dial only goes from 0 to 99"));
        }
        Ok(Dial(val))
    }

    fn turn(&self, turn: DialTurn) -> (Self, i32) {
        let amount = turn.amount();
        let mut rounds = amount / Self::MOD;

        let dial = if let DialTurn::Left(_) = turn {
            let turned = self.0 - (amount % Self::MOD);
            if turned <= Self::MIN && self.0 != Self::MIN {rounds += 1;}
            turned.rem_euclid(Self::MOD)
        } else {
            let turned = self.0 + (amount % Self::MOD);
            if turned > Self::MAX && self.0 != Self::MIN {rounds += 1;}
            turned.rem_euclid(Self::MOD)
        };

        (Self(dial), rounds)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }

    fn val(&self) -> i32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug)]
enum DialTurn {
    Left(i32),
    Right(i32),
}

impl DialTurn {
    fn amount(self) -> i32 {
        match self {
            Self::Left(n) => n,
            Self::Right(n) => n,
        }
    }
}

impl FromStr for DialTurn {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(L|R)([0-9][0-9]*)").unwrap();
        let (_, [direction, amount]) = re
            .captures(s)
            .ok_or_else(|| format!("{s} is not a valid dial turn"))?
            .extract();
        Ok(match direction {
            "L" => DialTurn::Left(amount.parse::<i32>()?),
            "R" => DialTurn::Right(amount.parse::<i32>()?),
            _ => unreachable!(),
        })
    }
}

fn solution(input: String) -> Result<(i32, i32), Box<dyn Error>> {
    let turns: Vec<DialTurn> = parse_vec(input)?;

    let (dial_part1, pw, pw4real) = turns
        .iter()
        .fold((Dial::new(50)?, 0, 0), |(dial, end_zeroes, all_zeroes), turn| {
            let (next, more_zeroes) = dial.turn(*turn);
            let end_zero = if next.is_zero() {1} else {0};
            (next, end_zeroes + end_zero, all_zeroes + more_zeroes)});

    Ok((pw, pw4real))
}

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input.txt")?;
    let mut r = BufReader::new(f);
    let mut input = String::new();
    r.read_to_string(&mut input)?;

    let (part1, part2) = solution(input)?;

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

#[test]
fn test() {
    let f = File::open("test_input.txt").unwrap();
    let mut r = BufReader::new(f);
    let mut input = String::new();
    r.read_to_string(&mut input).unwrap();

    assert_eq!(solution(input).unwrap(), (3, 6))
}

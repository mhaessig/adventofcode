use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Context {
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy)]
enum Instr {
    Do,
    Dont,
    Mul { lop: u64, rop: u64 },
}

impl Instr {
    pub fn from_str_parts(op: &str, lop: &str, rop: &str) -> Result<Self, Box<dyn Error>> {
        match op {
            "do" => Ok(Self::Do),
            "don't" => Ok(Self::Dont),
            "mul" => Ok(Self::Mul {
                lop: lop.parse()?,
                rop: rop.parse()?,
            }),
            _ => Err(format!("Unknown op \"{op}\"").into()),
        }
    }

    pub fn result(&self, ctx: &Context) -> (Option<u64>, Context) {
        match self {
            Self::Do => (None, Context { enabled: true }),
            Self::Dont => (None, Context { enabled: false }),
            Self::Mul { lop, rop } => {
                if ctx.enabled {
                    (Some(lop * rop), *ctx)
                } else {
                    (None, *ctx)
                }
            }
        }
    }
}

fn solution(r: BufReader<File>) -> Result<(u64, u64), Box<dyn Error>> {
    let mut mem = String::new();
    for line in r.lines() {
        let line = line?;
        mem.push_str(&line);
    }

    let mut muls = Vec::<Instr>::new();

    let re1 = Regex::new(r"mul\(([0-9]+),([0-9]+)\)")?;
    for (_, [lop, rop]) in re1.captures_iter(mem.as_str()).map(|c| c.extract()) {
        muls.push(Instr::from_str_parts("mul", lop, rop)?);
    }
    let part1 = muls
        .iter()
        .map(|mul| mul.result(&Context { enabled: true }).0.unwrap())
        .sum();

    let mut insn = Vec::<Instr>::new();
    let re2 = Regex::new(r"(do)\(()()\)|(don't)\(()()\)|(mul)\(([0-9]+),([0-9]+)\)")?;
    for (_, [op, lop, rop]) in re2.captures_iter(mem.as_str()).map(|c| c.extract()) {
        insn.push(Instr::from_str_parts(op, lop, rop)?);
    }

    let part2 = insn
        .iter()
        .fold(
            (Vec::<u64>::new(), Context { enabled: true }),
            |(mut results, ctx), instr| {
                let (res, new_ctx) = instr.result(&ctx);
                if let Some(res) = res {
                    results.push(res);
                }
                (results, new_ctx)
            },
        )
        .0
        .into_iter()
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

    assert_eq!(solution(r).unwrap(), (161, 48))
}

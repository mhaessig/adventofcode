extern crate nom;

use std::convert::TryFrom;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Clone, Copy, Debug)]
enum Insn {
    Acc(i32),
    Jmp(i32),
    JmpAbs(usize),
    Nop,
}

#[derive(Clone, Debug)]
struct Prog {
    prog: Vec<Insn>,
}

#[derive(Debug)]
struct State {
    ip: usize,
    acc: i32,
    hist: HashSet<usize>,
    stopped: bool,
    terminated: bool,
}

impl State {
    fn new() -> Self {
        Self {ip: 0, acc: 0, hist: HashSet::<usize>::new(), stopped: false, terminated: false}
    }

    fn step(mut self, prog: &Prog) -> Result<Self, Box<dyn Error>> {
        if self.hist.insert(self.ip) {
            let ins = &prog.prog[self.ip];
            match ins{
                Insn::Nop => self.ip +=1 ,
                Insn::Acc(n) => {self.acc += n; self.ip += 1},
                Insn::Jmp(n) => self.ip = usize::try_from(i32::try_from(self.ip)? + n)?,
                Insn::JmpAbs(u) => self.ip = *u,
            }

            // check if program has terminated
            if self.ip == prog.prog.len() {
                self.terminated = true;
            }
        } else {
            self.stopped = true;
        }

        Ok(self)
    }
}

impl Prog {
    fn parse<'a>(input: &'a str) -> nom::IResult<&'a str, Self, nom::error::VerboseError<&'a str>> {
        use nom::{
            branch::alt,
            bytes::complete::tag,
            character::complete::{newline, digit1, space1},
            combinator::{map, map_res, recognize},
            multi::separated_list1,
            sequence::{pair, preceded},
        };

        let parse_number = |i: &'a str| map_res(recognize(pair(alt((tag("+"), tag("-"))), digit1)), |s: &'a str| s.parse::<i32>())(i);
        let parse_acc = |i: &'a str| map(preceded(pair(tag("acc"), space1), parse_number), |n: i32| Insn::Acc(n))(i);
        let parse_jmp = |i: &'a str| map(preceded(pair(tag("jmp"), space1), parse_number), |n: i32| Insn::Jmp(n))(i);
        let parse_nop = |i: &'a str| map(preceded(pair(tag("nop"), space1), parse_number), |_| Insn::Nop)(i);
        let parse_insn = |i: &'a str| alt((parse_acc, parse_jmp, parse_nop))(i);
        let (i, p) = separated_list1(newline, parse_insn)(input)?;

        Ok((i, Prog{prog: p}))
    }

    fn transform_to_absolute(&self) -> Self {
        let transformed = self.prog.iter().enumerate().map(|(i, ins)| if let Insn::Jmp(n) = ins {
            Insn::JmpAbs(usize::try_from(i32::try_from(i).unwrap() + n).unwrap())
        } else {*ins}).collect();
        Prog {prog: transformed}
    }

    fn count_jmp(&self) -> i32 {
        self.prog.iter().fold(0, |sum, ins| if let Insn::Jmp(_) = ins { sum + 1 } else { sum })
    }

    fn k_jmp_to_nop(&self, k: i32) -> Self {
        let mut i = 0;
        let mut line: usize = 0;
        for (l, ins) in self.prog.iter().enumerate() {
            if let Insn::Jmp(_) = ins {
                if i == k {
                    line = l;
                    break;
                }

                i += 1;
            }
        }

        let mut new_prog = self.prog.clone();
        new_prog[line] = Insn::Nop;

        Prog {prog: new_prog}
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    let f = File::open("input.txt")?;
    let mut reader = BufReader::new(f);
    let mut text = String::new();
    reader.read_to_string(&mut text)?;

    let (_, program) = Prog::parse(&text).map_err(|e| display_error(e, &text))?;

    let mut next_state = State::new();
    while !next_state.stopped {
        next_state = next_state.step(&program)?;
    }

    println!("Part 1: accumulator = {}", next_state.acc);

    let num_jmp = program.count_jmp();
    println!("There are {} jumps", num_jmp);
    let mut s = State::new();
    for k in 0..num_jmp {
        let mod_prog = program.k_jmp_to_nop(k);
        s = State::new();

        while !(s.stopped || s.terminated) {
            s = s.step(&mod_prog)?;
        }

        if s.terminated {
            println!("k={}", k);
            break;
        }
    }

    println!("state: {:?}", s);
    println!("Part 2: accumulator = {}", s.acc);

    Ok(())
}

fn display_error(err: nom::Err<nom::error::VerboseError<&str>>, input: &str) -> String {
    match err {
        nom::Err::Error(e) | nom::Err::Failure(e) => {println!("Verbose error: {}", nom::error::convert_error(input, e)); String::from("see verbose error")},
        e => format!("{:?}", e)
    }
}

#[macro_use]
extern crate nom;
use nom::{
    IResult,
    bytes::complete::{tag, take_while1},
    character::complete::{anychar, digit1, space1},
};
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

struct PwPolicy {
    character: char,
    from: u8,
    to: u8,
    pw: String,
}

fn is_lowercase(chr: char) -> bool {
    chr.is_lowercase()
}

fn parse_line(line: &str) -> IResult<&str, PwPolicy> {
    named!(number<&str, u8>, map_res!(digit1, |s: &str| s.parse::<u8>()));

    // format: 3-3 a: asfasf
    let (i, from) = number(line)?;
    let (i, _) = tag("-")(i)?;
    let (i, to) = number(i)?;
    let (i, _) = space1(i)?;
    let (i, character) = anychar(i)?;
    let (i, _) = tag(": ")(i)?;
    let (i, pw_str) = take_while1(is_lowercase)(i)?;

    let pw = String::from(pw_str);

    Ok((i, PwPolicy{ character, from, to, pw }))
}

fn pw_is_valid_part1(policy: &PwPolicy) -> bool {
    let mut count = 0;
    for c in policy.pw.chars() {
        if c == policy.character {
            count += 1;
        }
    }

    count >= policy.from && count <= policy.to
}

fn pw_is_valid_part2(policy: &PwPolicy) -> bool {
    let chars = policy.pw.chars();
    let mut first = '0';
    let mut second = '0';

    for (i, c) in chars.enumerate() {
        if (policy.from - 1) as usize == i {
            first = c;
        }
        if (policy.to - 1) as usize  == i {
            second = c;
        }
    }

    let c = policy.character;

    (first == c && second != c) || (first != c && second == c)
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let buffer = BufReader::new(file);

    let mut counter1 = 0;
    let mut counter2 = 0;
    for line in buffer.lines() {
        let (_, policy) = parse_line(line.unwrap().as_str()).unwrap();

        if pw_is_valid_part1(&policy) {
            counter1 += 1;
        }

        if pw_is_valid_part2(&policy) {
            counter2 += 1;
        }
    }

    println!("Valid passwords part 1: {}", counter1);
    println!("Valid passwords part 2: {}", counter2);
}

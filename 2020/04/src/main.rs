#[macro_use]
extern crate nom;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_till1, take_while_m_n};
use nom::character::complete::{char, multispace1};
use nom::combinator::eof;
use nom::sequence::{pair, preceded, terminated};

use thiserror::Error;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::str::FromStr;

#[derive(Debug, Error)]
enum ParseError {
    #[error("not u16 height")]
    HeightError,
    #[error("unknown eye color")]
    EyeColorError
}

#[derive(Debug)]
enum Height {
    Cm(u16),
    In(u16)
}

impl FromStr for Height {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();
        let unit = &s[(len-2)..];
        let number = &s[..(len-2)].parse::<u16>();
        match (number, unit) {
            (Ok(h), "cm") => Ok(Height::Cm(*h)),
            (Ok(h), "in") => Ok(Height::In(*h)),
            _ => Err(ParseError::HeightError)
        }
    }
}

#[derive(Debug)]
enum EyeColor {
    Amber,
    Blue,
    Brown,
    Gray,
    Green,
    Hazel,
    Other
}

impl FromStr for EyeColor {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "amb" => Ok(Self::Amber),
            "blu" => Ok(Self::Blue),
            "brn" => Ok(Self::Brown),
            "gry" => Ok(Self::Gray),
            "grn" => Ok(Self::Green),
            "hzl" => Ok(Self::Hazel),
            "oth" => Ok(Self::Other),
            _ => Err(ParseError::EyeColorError)
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Passport {
    birth_year: u16,
    issue_year: u16,
    expiration_year: u16,
    height: Height,
    hair_color: String,
    eye_color: EyeColor,
    id: u64,
    country_id: Option<u16>
}

impl Passport {
    fn valid(&self) -> bool {
        self.birth_year >= 1920 && self.birth_year <= 2002 &&
            self.issue_year >= 2010 && self.issue_year <= 2020 &&
            self.expiration_year >= 2020 && self.expiration_year <= 2030 &&
            match self.height {
                Height::Cm(h) => (150..=193).contains(&h),
                Height::In(h) => (59..=76).contains(&h)
            }
    }
}

fn parse_field<'a>(input: &'a str, field_name: &str) -> nom::IResult<&'a str, &'a str> {
    terminated(
        preceded(
            pair(tag(field_name), char(':')),
            take_till1(|c: char| c.is_whitespace())
        ),
        alt((multispace1, eof))
    )(input)
}

fn parse_hcl<'a>(input: &'a str, field_name: &str) -> nom::IResult<&'a str, &'a str> {
    terminated(
        preceded(
            pair(tag(field_name), char(':')),
            preceded(char('#'), take_while_m_n(6, 6, |c: char| c.is_ascii_hexdigit()))
        ),
        alt((multispace1, eof))
    )(input)
}

fn parse_pid<'a>(input: &'a str, field_name: &str) -> nom::IResult<&'a str, &'a str> {
    terminated(
        preceded(
            pair(tag(field_name), char(':')),
            take_while_m_n(9, 9, |c: char| c.is_ascii_digit())
        ),
        alt((multispace1, eof))
    )(input)
}

fn parse_passport(input: &str) -> nom::IResult<&str, Passport> {
    named!(parse_birth_year<&str, u16>, map_res!(call!(parse_field, "byr"), |s: &str| s.parse::<u16>()));
    named!(parse_issue_year<&str, u16>, map_res!(call!(parse_field, "iyr"), |s: &str| s.parse::<u16>()));
    named!(parse_expiration_year<&str, u16>, map_res!(call!(parse_field, "eyr"), |s: &str| s.parse::<u16>()));
    named!(parse_height<&str, Height>, map_res!(call!(parse_field, "hgt"), |s: &str| s.parse::<Height>()));
    named!(parse_hair_color<&str, String>, map_res!(call!(parse_hcl, "hcl"), |s: &str| s.parse::<String>()));
    named!(parse_eye_color<&str, EyeColor>, map_res!(call!(parse_field, "ecl"), |s: &str| s.parse::<EyeColor>()));
    named!(parse_passport_id<&str, u64>, map_res!(call!(parse_pid, "pid"), |s: &str| s.parse::<u64>()));
    named!(parse_country_id<&str, u16>, map_res!(call!(parse_field, "cid"), |s: &str| s.parse::<u16>()));

    named!(passport_parser<&str, (u16, u16, u16, Height, String, EyeColor, u64, Option<u16>)>,
        permutation!(
            call!(parse_birth_year),
            call!(parse_issue_year),
            call!(parse_expiration_year),
            call!(parse_height),
            call!(parse_hair_color),
            call!(parse_eye_color),
            call!(parse_passport_id),
            call!(parse_country_id)?
    ));

    let (birth_year, issue_year, expiration_year, height, hair_color, eye_color, id, country_id) = passport_parser(input)?.1;

    Ok(("", Passport{ birth_year, issue_year, expiration_year, height, hair_color, eye_color, id, country_id }))
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf = BufReader::new(file);

    let mut input = String::new();
    let mut valid_counter = 0;

    for line in buf.lines() {
        let l = line.unwrap();

        if l.is_empty() && !input.is_empty() {
            let result= parse_passport(input.as_str());
            if let Ok((_, p)) = result { if p.valid() {valid_counter += 1} }

            input.clear();
        } else {
            input.push_str(l.as_str());
            input.push('\n');
        }
    }


    println!("Part 1: valid passports: {}", valid_counter);

}

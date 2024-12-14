#[macro_use]
extern crate nom;

use std::str;
use std::error::Error;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Clone, Debug)]
struct Bag {
    desc: String,
    contents: Option<HashMap<String, i32>>
}

impl Bag {
    fn parse(rule: &str) -> nom::IResult<&str, Self, nom::error::VerboseError<&str>> {
        use nom::{
            branch::alt,
            bytes::complete::tag,
            character::complete::{alpha1, digit1, space1},
            combinator::{map, map_res},
            error::{context, VerboseError},
            multi::separated_list1,
            sequence::{pair, terminated},
        };

        named!(bag_description<&str, String, VerboseError<&str>>, map!(count!(terminated!(call!(alpha1), call!(space1)), 2),
                |vec| [vec[0], " ", vec[1]].concat()));

        let (i, desc) = context("bag description", bag_description)(rule)?;
        let (i, _) = context("separator \"bags contain\"",
            pair(terminated(tag("bags"), space1), terminated(tag("contain"), space1))
        )(i)?;

        let (i, content) = context("list of contained bags", terminated(alt((
                context("case contains no bags", map(tag("no other bags"), |_| None)),
                context("case contains bags", map(separated_list1(terminated(tag(","), space1),
                    context("bag list item", terminated(
                        pair(map_res(terminated(digit1, space1), |s: &str| s.parse::<i32>()), bag_description), alt((tag("bags"), tag("bag")))
                    ))), Some))
                )),
            context("full stop at end of rule", tag("."))
        ))(i)?;

        let contents = content.map(make_contents);

        let bag = Self {desc, contents};

        Ok((i, bag))
    }
}

fn make_contents(vec: Vec<(i32, String)>) -> HashMap<String, i32> {
    let mut hm = HashMap::<String, i32>::new();
    for (num, desc) in vec {
        hm.insert(desc, num);
    }
    hm
}

fn display_error(err: nom::Err<nom::error::VerboseError<&str>>, input: &str) -> String {
    match err {
        nom::Err::Error(e) | nom::Err::Failure(e) => {println!("Verbose error: {}", nom::error::convert_error(input, e)); String::from("see verbose error")},
        e => format!("{:?}", e)
    }
}

fn count_containing(bags: HashMap<String, Bag>, bag: String) -> i32 {
    if let Some(b) =  bags.get(&bag) {
        match &b.contents {
            None => 0,
            Some(hm) => hm.iter()
                .fold(0, |aggr, (k, v)| aggr + v + v * count_containing(bags.clone(), k.clone()))
        }
    } else {
        0
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let buf = BufReader::new(file);

    let mut bags = HashMap::<String, Bag>::new();
    let mut contained_in = HashMap::<&str, Vec<&str>>::new();
    for l in buf.lines() {
        let line = l.unwrap();
        let (_, bag) = Bag::parse(line.as_str()).map_err(|e| display_error(e, line.as_str()))?;
        bags.insert(bag.desc.clone(), bag.clone());
    }

    for bag in bags.values() {
        match &bag.contents {
            None => (),
            Some(b) => {
                for k in b.keys() {
                    match contained_in.get_mut(k.as_str()) {
                        None => {contained_in.insert(k.as_str(), Vec::from([bag.desc.as_str()])); },
                        Some(v) => v.push(&*bag.desc)
                    }
                }
            }
        }
    }

    let mut shiny = HashSet::<&str>::new();
    if let Some(bs) = contained_in.get("shiny gold") {
        for b in bs {
            shiny.insert(b);
        }
    }

    let mut new = true;
    while new {
        let mut bools = Vec::<bool>::new();
        let mut hs = HashSet::<&str>::new();
        for s in shiny.iter() {
            if let Some(bs) = contained_in.get(s) {
                for b in bs {
                    hs.insert(b);
                }
            }
        }
        for b in hs {
            bools.push(shiny.insert(b));
        }

        new = bools.iter().any(|val| *val);
    }

    println!("Part 1: number of bags able to contain shiny gold bag: {}", shiny.len());
    println!("Part 2: number of bags a shiny gold bag contains: {}", count_containing(bags, String::from("shiny gold")));


    Ok(())
}

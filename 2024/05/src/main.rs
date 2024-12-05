use std::{
    collections::{BTreeSet, HashMap},
    error::Error,
    fs::{read_to_string, File},
    hash::Hash,
    io::{BufReader, Read},
    ops::Div,
};

use nom::{
    character::complete::{char, line_ending, u8},
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult, Parser,
};

#[derive(Debug, Clone)]
struct Page {
    pub number: u8,
    update_before: Vec<u8>,
    update_after: Vec<u8>,
}

impl Page {
    pub fn new(number: u8, update_before: &Vec<u8>, update_after: &Vec<u8>) -> Self {
        Page {
            number,
            update_before: update_before.clone(),
            update_after: update_after.clone(),
        }
    }
}

impl PartialEq for Page {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }

    fn ne(&self, other: &Self) -> bool {
        self.number != other.number
    }
}

impl Eq for Page {}

impl PartialOrd for Page {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Page {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self == other {
            return std::cmp::Ordering::Equal;
        } else if self.update_before.contains(&other.number) {
            std::cmp::Ordering::Less
        } else if self.update_after.contains(&other.number) {
            std::cmp::Ordering::Greater
        } else {
            unreachable!()
        }
    }
}

fn parse(s: &str) -> IResult<&str, (Vec<(u8, u8)>, Vec<Vec<u8>>)> {
    let relation = terminated(separated_pair(u8, char('|'), u8), line_ending);
    let print = terminated(separated_list1(char(','), u8), line_ending);
    separated_pair(many1(relation), line_ending, many1(print)).parse(s)
}

fn solution(input: String) -> Result<(u64, u64), Box<dyn Error>> {
    let Ok((residual, (relations, updates))) = parse(&input) else {
        return Err("Failed to parse input".into());
    };
    if residual != "" {
        return Err(format!("Failed to parse input completely. Residual: {residual}").into());
    }

    let mut update_before_map = HashMap::<u8, Vec<u8>>::new();
    let mut update_after_map = HashMap::<u8, Vec<u8>>::new();
    for (before, after) in relations.iter() {
        if let Some(v) = update_before_map.get_mut(before) {
            v.push(*after);
        } else {
            update_before_map.insert(*before, vec![*after]);
        }

        if let Some(v) = update_after_map.get_mut(after) {
            v.push(*before);
        } else {
            update_after_map.insert(*after, vec![*before]);
        }
    }

    let pages = relations
        .iter()
        .fold(BTreeSet::<u8>::new(), |mut ps, (p1, p2)| {
            ps.insert(*p1);
            ps.insert(*p2);
            ps
        })
        .iter()
        .map(|number| {
            (
                *number,
                Page::new(
                    *number,
                    update_before_map.get(number).unwrap_or(&Vec::new()),
                    update_after_map.get(number).unwrap_or(&Vec::new()),
                ),
            )
        })
        .collect::<HashMap<u8, Page>>();

    let updates = updates
        .iter()
        .map(|v| v.iter().map(|n| pages.get(n).unwrap()).collect())
        .collect::<Vec<Vec<&Page>>>();

    let part1 = updates
        .iter()
        .filter(|&update| update.is_sorted())
        .map(|update| update[update.len() / 2].number as u64)
        .sum();

    let part2 = updates
        .iter()
        .filter(|&update| !update.is_sorted())
        .map(|mut update| {
            let mut sorted = update.clone();
            sorted.sort();
            sorted[sorted.len() / 2].number as u64
        })
        .sum();

    Ok((part1, part2))
}

fn main() -> Result<(), Box<dyn Error>> {
    let (part1, part2) = solution(read_to_string("input.txt")?)?;

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

#[test]
fn test() {
    assert_eq!(
        solution(read_to_string("test_input.txt").unwrap()).unwrap(),
        (143, 123)
    )
}

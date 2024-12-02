use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input.txt")?;
    let r = BufReader::new(f);

    let (part1, part2) = solution(r)?;
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

struct OccuranceMap<T: Eq + Hash> {
    map: HashMap<T, u64>,
}

impl OccuranceMap<u64> {
    pub fn new(list: &[u64]) -> Self {
        let mut map = HashMap::<u64, u64>::new();

        for elem in list.iter() {
            if let Some(prev_val) = map.insert(*elem, 1) {
                map.insert(*elem, prev_val + 1);
            };
        }

        Self { map }
    }

    pub fn get(&self, k: &u64) -> u64 {
        if let Some(val) = self.map.get(k) {
            return *val;
        } else {
            return 0;
        }
    }
}

fn solution(input: BufReader<File>) -> Result<(u64, u64), Box<dyn Error>> {
    let mut left: Vec<u64> = Vec::new();
    let mut right: Vec<u64> = Vec::new();

    for (n, line) in input.lines().enumerate() {
        let line = line?;
        let mut parts = line.split_ascii_whitespace();
        let Some(str_l) = parts.next() else {
            return Err(format!("No left value on line {}", n).into());
        };
        let Some(str_r) = parts.next() else {
            return Err(format!("No right value on line {}", n).into());
        };

        left.push(str_l.parse()?);
        right.push(str_r.parse()?);
    }

    let om_r = OccuranceMap::new(&right);

    left.sort();
    right.sort();

    let part1 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum::<u64>();

    let part2 = left
        .iter()
        .map(|v| v * om_r.get(v))
        .sum();

    Ok((part1, part2))
}

#[test]
fn test() {
    let f = File::open("test_input.txt").unwrap();
    let r = BufReader::new(f);
    assert_eq!(solution(r).unwrap(), (11, 31));
}

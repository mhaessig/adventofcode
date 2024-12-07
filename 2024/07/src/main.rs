use std::{
    collections::VecDeque,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mul,
    Cat,
}

#[derive(Debug, Clone)]
struct Calibration {
    pub result: u64,
    pub vals: Vec<u64>,
}

impl Calibration {
    pub fn eval(&self, ops: Vec<Op>) -> u64 {
        assert!(ops.len() == self.vals.len() - 1);

        let mut res = self.vals[0];
        for i in 0..ops.len() {
            res = match ops[i] {
                Op::Add => res + self.vals[i + 1],
                Op::Mul => res * self.vals[i + 1],
                Op::Cat => format!("{res}{}", self.vals[i + 1]).parse().unwrap(),
            };
        }

        res
    }

    pub fn sat(&self, try_ops: Vec<Op>) -> bool {
        let mut all_ops: VecDeque<Vec<Op>> = try_ops.iter().map(|op| vec![*op]).collect();

        for i in 1..self.vals.len() - 1 {
            let to_pop = all_ops.len();
            for _ in 0..to_pop {
                if let Some(ops) = all_ops.pop_back() {
                    assert!(ops.len() == i);
                    for op in try_ops.iter() {
                        let mut ops_add = ops.clone();
                        ops_add.push(*op);
                        all_ops.push_front(ops_add);
                    }
                }
            }
        }

        for ops in all_ops {
            if self.result == self.eval(ops) {
                println!("{}", self.result);
                return true;
            }
        }

        false
    }
}

fn solution(r: BufReader<File>) -> Result<(u64, u64), Box<dyn Error>> {
    let mut cals = Vec::<Calibration>::new();

    for line in r.lines() {
        let line = line?;
        let (res_str, vals) = line.split_once(": ").unwrap();
        let vals = vals
            .split_whitespace()
            .map(|op| op.parse().unwrap())
            .collect();
        cals.push(Calibration {
            result: res_str.parse()?,
            vals,
        })
    }

    let part1 = cals
        .iter()
        .filter(|cal| cal.sat(vec![Op::Add, Op::Mul]))
        .map(|cal| cal.result)
        .sum();

    let part2 = cals
        .iter()
        .filter(|cal| cal.sat(vec![Op::Add, Op::Mul, Op::Cat]))
        .map(|cal| cal.result)
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

    assert_eq!(solution(r).unwrap(), (3749, 11387))
}

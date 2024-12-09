use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn in_bounds(&self, min: i32, max: i32) -> bool {
        min <= self.x && min <= self.y && self.x < max && self.y < max
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn all_pairs<T>(v: &Vec<T>) -> impl Iterator<Item = (&T, &T)> {
    v.iter()
        .enumerate()
        .map(|(i, elem)| v[i + 1..].iter().map(move |other| (elem, other)))
        .flatten()
}

fn solution(r: BufReader<File>) -> Result<(u64, u64), Box<dyn Error>> {
    let mut antennae = HashMap::<char, Vec<Point>>::new();
    let mut size = 0;

    for (line, y) in r.lines().zip(0i32..) {
        let line = line?;
        size = line.len();

        for (c, x) in line.chars().zip(0i32..) {
            if c.is_ascii_alphanumeric() {
                if let Some(v) = antennae.get_mut(&c) {
                    v.push(Point::new(x, y));
                } else {
                    antennae.insert(c, vec![Point::new(x, y)]);
                }
            }
        }
    }

    let mut antinodes = HashSet::<Point>::new();
    for points in antennae.values() {
        for (p1, p2) in all_pairs(points) {
            let diff = *p2 - *p1;
            antinodes.insert(*p2 + diff);
            antinodes.insert(*p1 - diff);

            if diff.x % 3 == 0 && diff.y % 3 == 0 {
                println!("inside");
                let diff3 = Point {
                    x: diff.x / 3,
                    y: diff.y / 3,
                };
                antinodes.insert(*p2 - diff3);
                antinodes.insert(*p1 + diff3);
            }
        }
    }

    let part1 = antinodes
        .iter()
        .filter(|p| p.in_bounds(0, size.try_into().unwrap()))
        .count()
        .try_into()?;

    let mut resonant_antinodes = HashSet::<Point>::new();
    for points in antennae.values() {
        for (p1, p2) in all_pairs(points) {
            let diff = *p2 - *p1;
            let mut new_point = *p1;
            while new_point.in_bounds(0, size.try_into().unwrap()) {
                resonant_antinodes.insert(new_point);
                new_point = new_point + diff;
            }

            let mut new_point = *p1;
            while new_point.in_bounds(0, size.try_into().unwrap()) {
                resonant_antinodes.insert(new_point);
                new_point = new_point - diff;
            }
        }
    }

    let part2 = resonant_antinodes.len().try_into()?;

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

    assert_eq!(solution(r).unwrap(), (14, 34))
}

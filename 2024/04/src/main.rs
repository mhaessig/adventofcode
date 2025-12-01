use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

#[derive(Debug, Clone, Copy)]
enum Direction {
    HorizontalForward,
    HorizontalBackward,
    VerticalUp,
    VerticalDown,
    DiagonalDownLeft,
    DiagonalDownRight,
    DiagonalUpLeft,
    DiagonalUpRight,
}

#[derive(Debug, Clone)]
struct SearchString {
    pub s: String,
    dir: Direction,
}

impl SearchString {
    pub fn new(s: String, dir: Direction) -> Self {
        SearchString { s, dir }
    }
}

fn solution(r: BufReader<File>) -> Result<(u64, u64), Box<dyn Error>> {
    let mut lines = Vec::<String>::new();
    for line in r.lines() {
        let line = line?;
        lines.push(line);
    }

    // All letter grids are square.
    let width = lines[0].len();
    let height = lines.len();
    assert!(width == height);

    // Build a list of search stings to search.
    // Scan horizontally, vertically, and diagonally forwards and backwards.
    // Then we can simply regex what we want.
    let mut search_strings = Vec::<SearchString>::with_capacity(8 * height);

    for line in lines.iter() {
        search_strings.push(SearchString::new(
            line.clone(),
            Direction::HorizontalForward,
        ));
        search_strings.push(SearchString::new(
            line.clone().chars().rev().collect(),
            Direction::HorizontalBackward,
        ));
    }

    for i in 0..width {
        let mut vert_line = String::with_capacity(height);
        for line in lines.iter().take(height) {
            vert_line.push_str(&line[i..=i]);
        }

        search_strings.push(SearchString::new(
            vert_line.clone(),
            Direction::VerticalDown,
        ));
        search_strings.push(SearchString::new(
            vert_line.clone().chars().rev().collect(),
            Direction::VerticalUp,
        ));
    }

    for diff in 0..(height - 3) {
        let mut s1 = String::with_capacity(height - diff);
        let mut s2 = String::with_capacity(height - diff);

        for i in 0..(height - diff) {
            let j = i + diff;
            s1.push_str(&lines[i][j..=j]);
            s2.push_str(&lines[j][i..=i]);
        }

        search_strings.push(SearchString::new(s1.clone(), Direction::DiagonalDownRight));
        search_strings.push(SearchString::new(
            s1.clone().chars().rev().collect(),
            Direction::DiagonalUpLeft,
        ));
        if diff > 0 {
            search_strings.push(SearchString::new(s2.clone(), Direction::DiagonalDownRight));
            search_strings.push(SearchString::new(
                s2.clone().chars().rev().collect(),
                Direction::DiagonalUpLeft,
            ));
        }
    }

    for diff in 3..height {
        let mut s1 = String::with_capacity(diff);
        let mut s2 = String::with_capacity(diff);

        for i in 0..=diff {
            let x1 = i;
            let y1 = diff - i;
            s1.push_str(&lines[x1][y1..=y1]);

            let x2 = height - 1 - diff + i;
            let y2 = height - 1 - i;
            s2.push_str(&lines[x2][y2..=y2]);
        }

        search_strings.push(SearchString::new(s1.clone(), Direction::DiagonalDownLeft));
        search_strings.push(SearchString::new(
            s1.clone().chars().rev().collect(),
            Direction::DiagonalUpRight,
        ));
        if diff < height - 1 {
            search_strings.push(SearchString::new(s2.clone(), Direction::DiagonalDownLeft));
            search_strings.push(SearchString::new(
                s2.clone().chars().rev().collect(),
                Direction::DiagonalUpRight,
            ));
        }
    }

    let re1 = Regex::new(r"XMAS")?;

    let mut xmas_count = 0;
    for search_str in search_strings.iter() {
        xmas_count += re1.captures_iter(&search_str.s).count();
    }

    let part1 = xmas_count as u64;

    // For part two move a 3x3 window over the entire grid and search that with an appropriate regex.
    let win_size = height - 3;
    let mut windows = Vec::<String>::with_capacity(win_size * win_size);
    for i in 0..=win_size {
        for j in 0..=win_size {
            let mut win = String::with_capacity(9);
            for k in 0..3 {
                win.push_str(&lines[i + k][j..(j + 3)]);
            }
            windows.push(win);
        }
    }

    let re2 = Regex::new(r"M.M.A.S.S|M.S.A.M.S|S.M.A.S.M|S.S.A.M.M")?;

    let mut x_mas_count = 0;
    for win in windows.iter() {
        x_mas_count += re2.captures_iter(win).count();
    }

    let part2 = x_mas_count as u64;

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

    assert_eq!(solution(r).unwrap(), (18, 9))
}

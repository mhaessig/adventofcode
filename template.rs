use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn solution(r: BufReader<File>) -> Result<(u64, u64), Box<dyn Error>> {

    for line in r.lines() {
        let line = line?;
    }

    let part1 = 0;
    let part2 = 0;

    Ok((part1, part2))
}

fn main() -> Result<(), Box<dyn Error>>{
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

    assert_eq!(solution(r).unwrap(), (0, 0))
}

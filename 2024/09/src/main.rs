use std::{
    collections::VecDeque,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy)]
enum Location {
    Free,
    File(u64),
}

fn solution(r: BufReader<File>) -> Result<(u64, u64), Box<dyn Error>> {
    let line = r.lines().flatten().collect::<String>();

    let mut mem = Vec::<Location>::new();
    for (c, i) in line.chars().zip(0u64..) {
        let n = c.to_string().parse::<u8>()?;

        for _ in 0..n {
            if i % 2 == 0 {
                mem.push(Location::File(i / 2));
            } else {
                mem.push(Location::Free);
            }
        }
    }

    let mut i = 0;
    let mut j = mem.len() - 1;
    while i < j {
        match mem[i] {
            Location::File(_) => {
                i += 1;
            }
            Location::Free => {
                while let Location::Free = mem[j] {
                    j -= 1;
                }
                mem[i] = mem[j];
                mem[j] = Location::Free;

                i += 1;
                j -= 1;
            }
        }
    }

    let part1 = mem.iter().zip(0u64..).fold(0, |sum, (loc, i)| {
        if let Location::File(id) = loc {
            sum + id * i
        } else {
            sum
        }
    });
    let part2 = 0;

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

    assert_eq!(solution(r).unwrap(), (1928, 0))
}

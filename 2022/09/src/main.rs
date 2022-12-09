use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    ops::Add,
    str::FromStr,
};

enum Move {
    Left(i64),
    Right(i64),
    Up(i64),
    Down(i64),
}

impl FromStr for Move {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        let n = parts[1]
            .parse::<i64>()
            .or_else(|_| Err("expected a number"))?;
        match parts[0] {
            "L" => Ok(Self::Left(n)),
            "R" => Ok(Self::Right(n)),
            "U" => Ok(Self::Up(n)),
            "D" => Ok(Self::Down(n)),
            _ => Err("not a move"),
        }
    }
}

fn move_tail(h: (i64, i64), t: (i64, i64)) -> (i64, i64) {
    let mut diff = (h.0 - t.0, h.1 - t.1);
    if diff.0.abs() <= 1 && diff.1.abs() <= 1 {
        return t;
    }

    if diff.0.abs() == 2 && diff.1.abs() == 2 {
        if diff.0 < 0 {
            diff.0 += 1;
        } else {
            diff.0 -= 1;
        }
        if diff.1 < 0 {
            diff.1 += 1;
        } else {
            diff.1 -= 1;
        }
    } else if diff.0.abs() == 2 && diff.1.abs() < 2 {
        if diff.0 < 0 {
            diff.0 += 1;
        } else {
            diff.0 -= 1;
        }
    } else if diff.1.abs() == 2 {
        if diff.1 < 0 {
            diff.1 += 1;
        } else {
            diff.1 -= 1;
        }
    } else {
        println!("head: {:?}, tail: {:?}", h, t);
        unreachable!();
    }

    return (t.0 + diff.0, t.1 + diff.1);
}

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input.txt")?;
    let r = BufReader::new(f);

    let mut moves = Vec::<Move>::new();
    for line in r.lines() {
        let line = line?;
        moves.push(Move::from_str(line.as_str())?);
    }

    let mut tail_positions = HashSet::<(i64, i64)>::new();
    let mut tail_positions2 = HashSet::<(i64, i64)>::new();
    let mut positions = [(0, 0); 10];

    for m in moves {
        let n = match m {
            Move::Down(n) | Move::Left(n) | Move::Right(n) | Move::Up(n) => n,
        };
        for _ in 0..n {
            match m {
                Move::Left(_) => {
                    positions[0].0 -= 1;
                }
                Move::Right(_) => {
                    positions[0].0 += 1;
                }
                Move::Up(_) => {
                    positions[0].1 += 1;
                }
                Move::Down(_) => {
                    positions[0].1 -= 1;
                }
            };

            println!("{}: position {:?}", 0, &positions[0]);
            for i in 1..10 {
                positions[i] = move_tail(positions[i - 1], positions[i]);
                println!("{}: position {:?}", i, &positions[i]);
            }
            tail_positions.insert(positions[1]);
            tail_positions2.insert(positions[9]);
        }
    }

    println!("The tail was at {} positions", tail_positions.len());
    println!("The long tail was at {} positions", tail_positions2.len());

    Ok(())
}

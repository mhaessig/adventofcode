use core::num;
use std::error::Error;

#[derive(Debug)]
enum Direction {
    Forward(usize),
    Down(usize),
    Up(usize),
}

#[derive(Debug)]
struct Position {
    forward: usize,
    depth: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    //let directions = include_str!("../example_input.txt")
        let directions = include_str!("../input.txt")
        .lines()
        .map(|s| s.split_at(s.find(' ').unwrap()))
        .map(|(dir, step)| {
            let num_steps = step[1..].parse().unwrap();
            match dir {
                "forward" => Direction::Forward(num_steps),
                "down" => Direction::Down(num_steps),
                "up" => Direction::Up(num_steps),
                _ => panic!("unknown direction {}", dir),
            }
        })
        .collect::<Vec<Direction>>();

    let pos = directions.iter().fold(
        Position {
            forward: 0,
            depth: 0,
        },
        |p, dir| match dir {
            Direction::Forward(n) => Position {
                forward: p.forward + n,
                depth: p.depth,
            },
            Direction::Down(n) => Position {
                forward: p.forward,
                depth: p.depth + n,
            },
            Direction::Up(n) => Position {
                forward: p.forward,
                depth: p.depth - n,
            },
        },
    );

    println!("Product of forward and depth: {}", pos.forward * pos.depth);

    let aim_pos = directions.into_iter().fold(
        (
            Position {
                forward: 0,
                depth: 0,
            },
            0,
        ),
        |(p, aim), dir| match dir {
            Direction::Down(n) => (p, aim + n),
            Direction::Up(n) => (p, aim - n),
            Direction::Forward(n) => (
                Position {
                    forward: p.forward + n,
                    depth: p.depth + aim * n,
                },
                aim,
            ),
        },
    );

    println!("Product of forward and depth: {}", aim_pos.0.forward * aim_pos.0.depth);

    Ok(())
}

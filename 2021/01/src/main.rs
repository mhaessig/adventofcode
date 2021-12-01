use itertools::Itertools;
use std::{error::Error};

enum Change {
    Increase,
    Decrease,
}

fn main() -> Result<(), Box<dyn Error>> {
    let measurements = include_str!("../input.txt")
        .lines()
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();

    let changes = measurements
        .windows(2)
        .map(|win| {
            if win[0] < win[1] {
                Change::Increase
            } else if win[0] > win[1] {
                Change::Decrease
            } else {
                panic!("Two measurements were equal!");
            }
        })
        .collect_vec();

    let incr_count = changes.iter().fold(0, |count, change| match change {
        Change::Increase => count + 1,
        Change::Decrease => count,
    });

    println!("Measurements increased {} times", incr_count);

    let sum_count = measurements
        .windows(3)
        .map(|win| win.iter().sum())
        .collect::<Vec<usize>>()
        .windows(2)
        .fold(0, |count, sums| {
            if sums[0] < sums[1] {
                count + 1
            } else {
                count
            }
        });

    println!("Sliding window sums increased {} times", sum_count);

    Ok(())
}

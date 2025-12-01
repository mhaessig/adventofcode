use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let mut numbers = HashSet::new();
    let file = File::open("input.txt").expect("File was not found");
    let reader = BufReader::new(file);
    let mut list: Vec<i64> = Vec::new();

    for line in reader.lines() {
        let number = line.unwrap().parse::<i64>().unwrap();

        list.push(number);
        numbers.insert(number);
    }

    // Part 1
    for n in &list {
        let complement = 2020 - n;
        if numbers.contains(&complement) {
            println!("Part 1: {}", n * complement);
            break;
        }
    }

    // Part 2
    let mut found = false;
    for n in &list {
        if found {
            break;
        }
        for m in &list {
            let complement = 2020 - m - n;
            if numbers.contains(&complement) {
                println!("Part 2: {}", m * n * complement);
                found = true;
                break;
            }
        }
    }
}

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf = BufReader::new(file);

    let mut counter: i128 = 0;
    let mut counters: Vec<i128> = vec!(0,0,0,0);
    let steps = vec!(1,3,5,7);
    let mut width = 0;
    for (i, line) in buf.lines().enumerate() {
        if i == 0 {
            width = line.unwrap().len();
            continue;
        }

        let l = line.unwrap();

        for (j, m) in steps.iter().enumerate() {
            let n = ((i * m) % width) as usize;
            if '#' == l.chars().nth(n).unwrap() {
                counters[j] += 1;
            }
        }

        if i % 2 == 0 {
            let n = ((i / 2) % width) as usize;
            if '#' == l.chars().nth(n).unwrap() {
                counter += 1;
            }
        }
    }

    let part1 = counters[1];
    let part2 = counters.iter().fold(counter, |a, b| a * b);
    println!("counters: {:?},{}",counters,counter);

    println!("Part 1: number of trees: {}", part1);
    println!("Part 2: muliplication: {}", part2);
}

use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader}
};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input.txt")?;
    let r = BufReader::new(f);

    let mut calories = Vec::<u64>::new();

    let mut sum: u64 = 0;
    for line in r.lines() {
        let line = line?;
        if line.is_empty() {
            calories.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<u64>()?;
        }
    }

    calories.sort();
    calories.reverse();
    println!("Max calories: {}", calories[0]);
    println!("Max backup calories: {}", calories.iter().take(3).sum::<u64>());

    Ok(())
}

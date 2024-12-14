
use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input.txt")?;
    let r = BufReader::new(f);

    let mut rucksacks = Vec::<(HashSet<char>, HashSet<char>)>::new();

    for line in r.lines() {
        let line = line?;
        let mut compartment1 = HashSet::<char>::new();
        let mut compartment2 = HashSet::<char>::new();

        compartment1.extend(line.chars().take(line.len() / 2));
        compartment2.extend(line.chars().skip(line.len() / 2));

        rucksacks.push((compartment1, compartment2));
    }

    let prio_fun = |p: char| {
        if p.is_ascii_lowercase() {
            p as u32 - 0x60
        } else {
            p as u32 - (0x41 - 27)
        }
    };

    let prio_sum = rucksacks.clone()
        .into_iter()
        .map(|(c1, c2)| c1.intersection(&c2).next().unwrap().to_owned())
        .map(prio_fun)
        .sum::<u32>();

    println!("Sum of priorities: {}", prio_sum);

    let groups = rucksacks
        .chunks(3)
        .map(|a| {
            let a1 = a[0].0.union(&a[0].1).copied().collect::<HashSet<char>>();
            let a2 = a[1].0.union(&a[1].1).copied().collect::<HashSet<char>>();
            let a3 = a[2].0.union(&a[2].1).copied().collect::<HashSet<char>>();
            let inter = a2.intersection(&a3).copied().collect::<HashSet<char>>();
            a1.clone().intersection(&inter.clone())
        }.next()
                .unwrap()
                .to_owned()
        )
        .map(prio_fun)
        .sum::<u32>();

    println!("Sum of priorities: {}", groups);

    Ok(())
}

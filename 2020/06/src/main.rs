use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let buf = BufReader::new(file);

    let mut sum1 = 0;
    let mut sum2 = 0;
    let mut answers = HashSet::<char>::new();
    let mut group_answers = Vec::<Box<HashSet<char>>>::new();

    for l in buf.lines() {
        let line = l.unwrap();
        if line == "" {
            sum1 += answers.len();
            answers.clear();

            let mut ans = *(group_answers.pop().unwrap());
            for hs in group_answers.iter() {
                let tmp = ans.clone();
                ans.clear();
                for c in tmp.intersection(&*hs) {
                    ans.insert(*c);
                }
            }
            sum2 += ans.len();
            group_answers.clear();
        } else {
            let mut person_answers = HashSet::<char>::new();
            for c in line.chars() {
                answers.insert(c);
                person_answers.insert(c);
            }
            group_answers.push(Box::new(person_answers));
        }
    }

    println!("Part 1: number of yes: {}", sum1);
    println!("Part 2: number of all yes: {}", sum2);

    Ok(())
}

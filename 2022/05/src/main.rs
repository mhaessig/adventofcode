use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy)]
struct Instruction {
    from: usize,
    to: usize,
    amount: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input.txt")?;
    let r = BufReader::new(f);

    let mut stacks = Vec::<Vec<char>>::new();

    let mut instruction_desc = String::new();
    let mut stack_desc = String::new();
    let mut instr_desc = false;
    for line in r.lines() {
        let line = line?;
        if line.is_empty() {
            instr_desc = true;
        }

        if instr_desc {
            if instruction_desc.is_empty() {
                instruction_desc = line;
            } else {
                instruction_desc = format!("{}\n{}", instruction_desc, line);
            }
        } else if stack_desc.is_empty() {
            stack_desc = line;
        } else {
            stack_desc = format!("{}\n{}", stack_desc, line);
        }
    }

    let num_stacks = format!(
        "{}",
        stack_desc
            .split('\n').next_back()
            .unwrap()
            .chars()
            .filter(|c| c.is_ascii_digit())
            .last()
            .unwrap()
    )
    .parse::<usize>()?;

    for i in 0..num_stacks {
        stacks.push(Vec::<char>::new());
        for line in stack_desc.split('\n').rev().skip(1) {
            let c = line.chars().nth(i * 4 + 1).unwrap();
            if c.is_ascii_alphabetic() {
                stacks[i].push(c);
            }
        }
    }

    let instructions = instruction_desc
        .lines()
        .map(|l| l.split(' '))
        .map(|mut splits| {
            let amount = splits.nth(1).unwrap().parse::<usize>().unwrap();
            let from = splits.nth(1).unwrap().parse::<usize>().unwrap();
            let to = splits.nth(1).unwrap().parse::<usize>().unwrap();
            Instruction { from, to, amount }
        })
        .collect::<Vec<_>>();

    let mut stacks9001 = stacks.clone();

    for instr in &instructions {
        for _ in 0..instr.amount {
            let c = stacks[instr.from - 1].pop();
            if let Some(c) = c {
                stacks[instr.to - 1].push(c);
            }
        }
    }

    print!("On top: ");
    for st in stacks {
        if let Some(c) = st.last() {
            print!("{}", c);
        } else {
            print!(" ");
        }
    }
    println!();


    for instr in &instructions {
        let mut lift = Vec::<char>::new();
        for _ in 0..instr.amount{
            let c = stacks9001[instr.from - 1].pop();
            if let Some(c) = c {
                lift.insert(0, c);
            }
        }

        stacks9001[instr.to - 1].append(&mut lift);
    }

    print!("On top: ");
    for st in stacks9001 {
        if let Some(c) = st.last() {
            print!("{}", c);
        } else {
            print!(" ");
        }
    }
    println!();


    Ok(())
}

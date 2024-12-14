use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input.txt")?;
    let r = BufReader::new(f);

    let mut x = 1;
    let mut cycles = 1;
    let cycles_of_interest = [20, 60, 100, 140, 180, 220, i32::MAX];
    let mut i = 0;
    let mut sum = 0;

    for line in r.lines() {
        let line = line?;
        let instr = &line.as_str()[0..4];

        print_crt(cycles, x);
        cycles += 1;
        sum += update(cycles, x, cycles_of_interest[i]);
        if cycles == cycles_of_interest[i] {
            i += 1;
        }

        match instr {
            "noop" => (),
            "addx" => {
                print_crt(cycles, x);
                cycles += 1;
                let number = line.as_str()[5..].parse::<i32>()?;
                x += number;
            }
            _ => unreachable!(),
        };

        sum += update(cycles, x, cycles_of_interest[i]);
        if cycles == cycles_of_interest[i] {
            i += 1;
        }
    }

    println!("Signal strenth sum: {}", sum);

    Ok(())
}

fn update(cycles: i32, x: i32, interest: i32) -> i32 {
    if cycles == interest {
        cycles * x
    } else {
        0
    }
}

fn print_crt(cycle: i32, x: i32) {
    if cycle > 240 {
        return;
    }

    let pos = (cycle - 1) % 40;

    if pos >= x - 1 && pos <= x + 1 {
        print!("#");
    } else {
        print!(" ");
    }

    if pos == 39 {
        println!();
    }
}

type Octos = [[u32; 10]; 10];

fn do_flash(i: usize, j: usize) -> Octos {
    let mut changes = Octos::default();

    if i > 0 {
        changes[i - 1][j] += 1;
        if j > 0 {
            changes[i - 1][j - 1] += 1;
        }
        if j < 9 {
            changes[i - 1][j + 1] += 1;
        }
    }
    if i < 9 {
        changes[i + 1][j] += 1;
        if j > 0 {
            changes[i + 1][j - 1] += 1;
        }
        if j < 9 {
            changes[i + 1][j + 1] += 1;
        }
    }

    if j > 0 {
        changes[i][j - 1] += 1;
    }
    if j < 9 {
        changes[i][j + 1] += 1;
    }

    changes
}

fn add_octos(o1: &Octos, o2: &Octos) -> Octos {
    let mut sum = Octos::default();
    for i in 0..10 {
        for j in 0..10 {
            sum[i][j] = o1[i][j] + o2[i][j];
        }
    }

    sum
}

fn add_octos_truncated(o1: &Octos, o2: &Octos) -> Octos {
    let mut sum = Octos::default();
    for i in 0..10 {
        for j in 0..10 {
            let s = o1[i][j] + o2[i][j];
            if s > 10 && o1[i][j] < 10 {
                sum[i][j] = 10;
            } else if o1[i][j] == 10 {
                sum[i][j] = 11;
            } else {
                sum[i][j] = s;
            }
        }
    }

    sum
}

fn step(o: &mut Octos) -> u32 {
    // first, increase energy of every octopus by one
    for i in 0..10 {
        for j in 0..10 {
            o[i][j] += 1;
        }
    }

    // then, let them flash until they are done
    let mut sth_changed = true;
    while sth_changed {
        sth_changed = false;
        let mut changes = Octos::default();

        for i in 0..10 {
            for j in 0..10 {
                if o[i][j] == 10 {
                    changes = add_octos(&changes, &do_flash(i, j));
                    sth_changed = true;
                }
            }
        }

        if sth_changed {
            *o = add_octos_truncated(o, &changes);
        }
    }

    // count the flashes (energies > 9, because an octopus can only flash once per step)
    // all energies > 9 must be set to 0
    let mut flashes = 0;
    for i in 0..10 {
        for j in 0..10 {
            if o[i][j] > 9 {
                o[i][j] = 0;
                flashes += 1;
            }
        }
    }

    flashes
}

fn main() {
    let lines = include_str!("../example_input.txt").lines();
    //let lines = include_str!("../input.txt").lines();

    let mut o = Octos::default();
    for (i, s) in lines.enumerate() {
        for (j, c) in s.chars().enumerate() {
            o[i][j] = c.to_digit(10).unwrap();
        }
    }

    let mut flashes = 0;
    for i in 0.. {
        if i == 100 {
            dbg!(flashes);
        }
        let new_flashes = step(&mut o);
        flashes += new_flashes;

        if new_flashes == 100 {
            dbg!(i+1);
            break;
        }
    }

}

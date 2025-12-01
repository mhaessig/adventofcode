use std::collections::HashSet;

enum Fold {
    Left(isize),
    Up(isize),
}

fn main() {
    let lines = include_str!("../input.txt").lines();
    //let lines = include_str!("../example_input.txt").lines();

    let mut coords: HashSet<(isize, isize)> = HashSet::new();
    let mut instrs: Vec<Fold> = Vec::new();
    let mut empty_line = false;
    for l in lines {
        if l.is_empty() {
            empty_line = true;
            continue;
        }

        if empty_line {
            match l
                .replace("fold along ", "")
                .as_str()
                .split_once('=')
                .unwrap()
            {
                ("x", n) => instrs.push(Fold::Left(n.parse().unwrap())),
                ("y", n) => instrs.push(Fold::Up(n.parse().unwrap())),
                _ => unreachable!(),
            };
        } else {
            let (x, y) = l.split_once(',').unwrap();
            coords.insert((x.parse().unwrap(), y.parse().unwrap()));
        }
    }

    for i in instrs {
        let mut new_coords = HashSet::new();
        for (x, y) in coords {
            match i {
                Fold::Left(foldx) if foldx <= x => new_coords.insert((x - 2 * (x - foldx), y)),
                Fold::Up(foldy) if foldy <= y => new_coords.insert((x, y - 2 * (y - foldy))),
                _ => new_coords.insert((x, y)),
            };
        }

        coords = new_coords;
        dbg!(&coords.len());
    }

    let mut max = (0,0);
    for (x, y) in coords.iter() {
        if *x > max.0 {
            max.0 = *x;
        }
        if *y > max.1 {
            max.1 = *y;
        }
    }

    for j in 0..(max.1 + 1) {
        let mut s = String::new();
        for i in 0..(max.0 + 1){
            if coords.contains(&(i,j)) {
                s = format!("{}{}", s, "#");
            } else {
                s = format!("{}{}", s, " ");
            }
        }
        println!("{}", s);
    }

}

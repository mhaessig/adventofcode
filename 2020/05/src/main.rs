use std::convert::TryInto;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf = BufReader::new(file);

    let mut max_id = -1;
    let mut all_seats: Vec<i32> = (0..(8*127+7)).collect();

    for l in buf.lines() {
        let line = l.unwrap();
        let mut row = 0;
        let mut col = 0;
        println!("line {}", line);
        for (i,c) in line.chars().enumerate() {
            println!("{} row {:?} col {:?}", c, row, col);
            if i < 7 && c == 'B' {
                row += 2_i32.pow((6 - i).try_into().unwrap());
            } else if i >= 7 && c == 'R' {
                col += 2_i32.pow((2 - (i - 7)).try_into().unwrap());
            }
        }

        let seat_id = row * 8 + col;
        println!("row {}, col {}, id {}", row, col, seat_id);

        if seat_id > max_id {
            max_id = seat_id;
        }

        let index = all_seats.iter().position(|s| *s == seat_id).unwrap();
        all_seats.remove(index);
    }

    println!("Part 1: max seat id: {}", max_id);
    println!("Part 2: {:?}", all_seats);
}

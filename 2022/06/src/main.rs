use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input.txt")?;
    let r = BufReader::new(f);

    let datastream = r.lines().next().unwrap().unwrap();

    find_start_marker(&datastream, 4);
    find_start_marker(&datastream, 14);

    Ok(())
}

fn find_start_marker(datastream: &str, len: usize) {
    for i in 0..(datastream.len() - len) {
        let slice = &datastream[i..i + len];
        let mut all_different = true;
        for (j, c) in slice.chars().enumerate() {
            for k in 0..len {
                if k == j {
                    continue;
                }
                all_different = all_different && c != slice.chars().nth(k).unwrap();
            }
        }

        if all_different {
            println!("Marker after {} characters", i + len);
            break;
        }
    }
}

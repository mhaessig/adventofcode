use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Lines},
};

#[derive(Clone, Debug)]
#[allow(dead_code)]
enum Fs {
    File { name: String, size: u64 },
    Dir { name: String, children: Vec<Self> },
}

impl Fs {
    fn size(&self) -> u64 {
        match self {
            Self::File { name: _, size } => *size,
            Self::Dir { name: _, children } => children.iter().fold(0, |sum, fs| sum + fs.size()),
        }
    }
}

fn parse_dir(lines: &mut Lines<BufReader<File>>, name: String) -> Fs {
    let mut children = Vec::<Fs>::new();

    while let Some(Ok(line)) = lines.next() {
        match line.split(" ").collect::<Vec<_>>()[..] {
            ["$", "ls"] => continue,
            ["$", "cd", ".."] => break,
            ["$", "cd", name] => children.push(parse_dir(lines, name.to_string())),
            ["dir", _] => continue,
            [size, name] => children.push(Fs::File {
                name: name.to_string(),
                size: size.parse::<u64>().unwrap(),
            }),
            _ => unreachable!(),
        };
    }

    Fs::Dir { name, children }
}

fn count_below_size((size, count): (u64, u64), dir: Fs, max_size: u64) -> (u64, u64) {
    let dir_size = dir.size();
    match dir {
        Fs::Dir { name: _, children } => children.into_iter().fold(
            if dir_size < max_size {
                (dir_size + size, count + 1)
            } else {
                (size, count)
            },
            |tuple, fs| count_below_size(tuple, fs, max_size),
        ),
        _ => (size, count),
    }
}

fn dir_to_delete(size: u64, dir: Fs, min_size: u64) -> u64 {
    let dir_size = dir.size();
    match dir {
        Fs::Dir { name: _, children } => children.into_iter().fold(
            if dir_size >= min_size && dir_size < size {
                dir_size
            } else {
                size
            },
            |to_delete, fs| dir_to_delete(to_delete, fs, min_size),
        ),
        _ => size,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input.txt")?;
    let r = BufReader::new(f);

    let mut lines = r.lines();
    let Some(Ok(line)) = lines.next() else {
        unreachable!();
    };
    let fs = match line.split(" ").collect::<Vec<_>>()[..] {
        ["$", "cd", name] => parse_dir(&mut lines, name.to_string()),
        _ => unreachable!(),
    };

    let (size, count) = count_below_size((0, 0), fs.clone(), 100000);
    println!("{} Directories below 100000 have size {}", count, size);

    let fs_size = fs.size();
    let size_required = 30000000 - (70000000 - fs_size);

    let size_to_delete = dir_to_delete(u64::MAX, fs, size_required);
    println!("Directory to delete has size {}", size_to_delete);

    Ok(())
}

use std::{
    error::Error,
    fmt::Display,
    fs,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Location {
    Free,
    File { id: u64, size: usize },
}

impl Location {
    pub fn is_file(&self) -> bool {
        matches!(self, Self::File { id: _, size: _ })
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Location::File { id, size: _ } => write!(f, "{id}"),
            Location::Free => write!(f, "."),
        }
    }
}

struct File {
    idx: usize,
    id: u64,
    size: usize,
}

#[derive(Debug)]
struct Free {
    idx: usize,
    size: usize,
}

fn solution(r: BufReader<fs::File>) -> Result<(u64, u64), Box<dyn Error>> {
    let line = r.lines().map_while(Result::ok).collect::<String>();

    let mut mem = Vec::<Location>::new();
    let mut files = Vec::<File>::new();
    let mut free_list= Vec::<Free>::new();
    for (c, i) in line.chars().zip(0u64..) {
        let n = c.to_string().parse::<usize>()?;

        if i % 2 == 0 {
            files.push(File { idx: mem.len(), id: i / 2, size: n });
            for _ in 0..n {
                mem.push(Location::File { id: i / 2, size: n });
            }
        } else {
            free_list.push(Free{idx: mem.len(), size: n});
            for _ in 0..n {
                mem.push(Location::Free);
            }
        }
    }
    let mem = mem;

    let mut i = 0;
    let mut j = mem.len() - 1;
    let mut mem_frag = mem.clone();
    while i < j {
        match mem_frag[i] {
            Location::File { id: _, size: _ } => {
                i += 1;
            }
            Location::Free => {
                while let Location::Free = mem_frag[j] {
                    j -= 1;
                }
                mem_frag[i] = mem_frag[j];
                mem_frag[j] = Location::Free;

                i += 1;
                j -= 1;
            }
        }
    }

    let part1 = mem_frag.iter().zip(0u64..).fold(0, |sum, (loc, i)| {
        if let Location::File { id, size: _ } = loc {
            sum + id * i
        } else {
            sum
        }
    });

    let mut mem_defrag = mem.clone();
    for f in files.iter().rev() {
        for free in free_list.iter_mut() {
            if free.size >= f.size && free.idx < f.idx {
                for i in free.idx..free.idx+f.size {
                    mem_defrag[i] = mem_defrag[f.idx];
                }
                free.size -= f.size;
                free.idx += f.size;

                for loc in mem_defrag.iter_mut().skip(f.idx).take(f.size) {
                    *loc = Location::Free;
                }
                break;
            }
        }
    }

    let part2 = mem_defrag.iter().zip(0u64..).fold(0, |sum, (loc, i)| {
        if let Location::File { id, size: _ } = loc {
            sum + id * i
        } else {
            sum
        }
    });

    Ok((part1, part2))
}

fn main() -> Result<(), Box<dyn Error>> {
    let f = fs::File::open("input.txt")?;
    let r = BufReader::new(f);

    let (part1, part2) = solution(r)?;

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

#[test]
fn test() {
    let f = fs::File::open("test_input.txt").unwrap();
    let r = BufReader::new(f);

    assert_eq!(solution(r).unwrap(), (1928, 2858))
}

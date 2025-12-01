use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    mem,
};

#[derive(Clone, Copy)]
struct Interval<T>
where
    T: Ord + Copy,
{
    lower: T,
    upper: T,
}

impl<T: Ord + Copy> Interval<T> {
    fn new(a: T, b: T) -> Self {
        let mut lower = a;
        let mut upper = b;
        if lower > upper {
            mem::swap(&mut lower, &mut upper);
        }

        Interval { lower, upper }
    }

    fn contains(&self, other: &Self) -> bool {
        self.lower <= other.lower && self.upper >= other.upper
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.contains(other)
            || other.contains(self)
            || (self.lower >= other.lower && self.upper >= other.upper && self.lower <= other.upper)
            || (self.lower <= other.lower && self.upper <= other.upper && self.upper >= other.lower)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input.txt")?;
    let r = BufReader::new(f);

    let mut assignments = Vec::<(Interval<u32>, Interval<u32>)>::new();

    for line in r.lines() {
        let line = line?;

        let (team1, team2) = line.split_once(',').unwrap();

        let intervals = [team1, team2]
            .into_iter()
            .map(|s| s.split_once('-').unwrap())
            .map(|(lower, upper)| {
                [lower, upper]
                    .into_iter()
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .map(|v| Interval::new(v[0], v[1]))
            .collect::<Vec<Interval<u32>>>();

        assignments.push((intervals[0], intervals[1]))
    }

    let containing = assignments
        .iter()
        .map(|(team1, team2)| team1.contains(team2) || team2.contains(team1))
        .fold(0, |sum, a| if a { sum + 1 } else { sum });

    println!("Fully containing: {}", containing);

    let overlapping = assignments
        .iter()
        .map(|(team1, team2)| team1.overlaps(team2))
        .fold(0, |sum, a| if a { sum + 1 } else { sum });

    println!("Overlapping: {}", overlapping);

    Ok(())
}

use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn play(&self, other: &Self) -> Outcome {
        if self == other {
            return Outcome::Draw;
        }

        match (self, other) {
            (Self::Rock, Self::Scissors)
            | (Self::Paper, Self::Rock)
            | (Self::Scissors, Self::Paper) => Outcome::Win,
            _ => Outcome::Loose,
        }
    }

    fn value(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

fn draw_for_outcome(outcome: &Outcome, opponent: &Hand) -> Hand {
    match outcome {
        Outcome::Win => match opponent {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        },
        Outcome::Loose => match opponent {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        },
        Outcome::Draw => *opponent
    }
}

impl FromStr for Hand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err("Expected A,B,C,X,Y, or Z"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Outcome {
    Win,
    Loose,
    Draw,
}

impl Outcome {
    fn value(&self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Loose => 0,
        }
    }
}

impl FromStr for Outcome {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Loose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err("Expected X, Y, or Z")
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input.txt")?;
    let r = BufReader::new(f);

    let mut rounds = Vec::<(Hand, Hand, Outcome)>::new();

    for line in r.lines() {
        let line = line?;
        rounds.push((
            Hand::from_str(&line[2..3])?,
            Hand::from_str(&line[0..1])?,
            Outcome::from_str(&line[2..3])?,
        ));
    }

    println!(
        "Score: {}",
        rounds.iter().map(|(us, other, _)| us.play(other).value() + us.value()).sum::<u32>()
    );

    println!("Score: {}", rounds.iter().map(|(_, other, outcome)| draw_for_outcome(outcome, other).value() + outcome.value()).sum::<u32>());

    Ok(())
}

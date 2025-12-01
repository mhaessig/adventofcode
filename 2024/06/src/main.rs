use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    pub x: i64,
    pub y: i64,
}

impl Position {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn next_in_dir(&self, dir: Direction) -> Self {
        let (x, y) = match dir {
            Direction::Up => (self.x, self.y - 1),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
            Direction::Right => (self.x + 1, self.y),
        };
        Self { x, y }
    }

    pub fn surrounding(&self) -> HashSet<Self> {
        let mut set = HashSet::new();

        for x in self.x - 1..=self.x + 1 {
            for y in self.y - 1..=self.y + 1 {
                if x == y {
                    continue;
                }
                set.insert(Position::new(x, y));
            }
        }

        set
    }
}

#[derive(Debug)]
enum GuardAction {
    Step,
    TurnRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Guard {
    pub pos: Position,
    dir: Direction,
}

impl Guard {
    pub fn new(x: i64, y: i64, dir: Direction) -> Self {
        Self {
            pos: Position::new(x, y),
            dir,
        }
    }

    pub fn step(self, area: &Area) -> Option<(Self, GuardAction)> {
        let new_pos = self.pos.next_in_dir(self.dir);

        if !area.pos_is_inside(&new_pos) {
            return None;
        }

        if area.pos_is_obstacle(&new_pos) {
            Some((
                Self {
                    pos: self.pos,
                    dir: self.dir.turn_right(),
                },
                GuardAction::TurnRight,
            ))
        } else {
            Some((
                Self {
                    pos: new_pos,
                    dir: self.dir,
                },
                GuardAction::Step,
            ))
        }
    }
}

// The first position is (1,1) and the last is (size, size).
#[derive(Debug)]
struct Area {
    pub size: u32,
    pub obstacles: HashSet<Position>,
}

impl Area {
    pub fn new(size: u32, obstacles: HashSet<Position>) -> Self {
        Self { size, obstacles }
    }

    pub fn pos_is_inside(&self, pos: &Position) -> bool {
        pos.x > 0 && pos.y > 0 && pos.x <= self.size.into() && pos.y <= self.size.into()
    }

    pub fn pos_is_obstacle(&self, pos: &Position) -> bool {
        self.obstacles.contains(pos)
    }

    pub fn with_additional_obstacle(&self, pos: Position) -> Self {
        let mut new_obss = self.obstacles.clone();
        new_obss.insert(pos);
        Self::new(self.size, new_obss)
    }
}

fn solution(r: BufReader<File>) -> Result<(u64, u64), Box<dyn Error>> {
    let mut obstacles = HashSet::new();
    let mut size: u32 = 0;
    let mut guard = Guard::new(0, 0, Direction::Up);

    for (y, line) in r.lines().enumerate() {
        let line = line?;
        if y == 0 {
            size = line.len().try_into()?;
        }

        for (x, c) in line.chars().enumerate() {
            let x: i64 = (x + 1).try_into()?;
            let y: i64 = (y + 1).try_into()?;

            match c {
                '.' => (),
                '#' => _ = obstacles.insert(Position::new(x, y)),
                '^' => guard = Guard::new(x, y, Direction::Up),
                'v' => guard = Guard::new(x, y, Direction::Down),
                '>' => guard = Guard::new(x, y, Direction::Right),
                '<' => guard = Guard::new(x, y, Direction::Left),
                _ => return Err(format!("Unexpected character '{c}'").into()),
            };
        }
    }

    let start_guard = guard;
    let area = Area::new(size, obstacles);
    let mut guard_history = Vec::<Position>::new();
    guard_history.push(guard.pos);

    while let Some((new_guard, _)) = guard.step(&area) {
        guard = new_guard;
        guard_history.push(guard.pos);
    }

    let _ = guard;
    guard_history.dedup();

    let part1 = guard_history
        .clone()
        .into_iter()
        .collect::<HashSet<Position>>()
        .len()
        .try_into()?;

    let mut loop_obss = HashSet::<Position>::new();
    for additional_obs in guard_history[1..guard_history.len() - 1]
        .iter()
        .filter(|pos| area.pos_is_inside(pos))
    {
        let mod_area = area.with_additional_obstacle(*additional_obs);
        let mut guard = start_guard;
        let mut sentinel_guards = vec![start_guard].into_iter().collect::<HashSet<Guard>>();

        while let Some((new_guard, action)) = guard.step(&mod_area) {
            guard = new_guard;
            if sentinel_guards.contains(&guard) {
                loop_obss.insert(*additional_obs);
                break;
            }
            if let GuardAction::TurnRight = action {
                sentinel_guards.insert(guard);
            }
        }
    }

    let part2 = loop_obss.len().try_into()?;

    Ok((part1, part2))
}

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input.txt")?;
    let r = BufReader::new(f);

    let (part1, part2) = solution(r)?;

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

#[test]
fn test() {
    let f = File::open("test_input.txt").unwrap();
    let r = BufReader::new(f);

    assert_eq!(solution(r).unwrap(), (41, 6))
}

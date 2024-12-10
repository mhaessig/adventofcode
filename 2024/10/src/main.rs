use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    vec,
};

type TopoMap = Vec<Vec<u8>>;
type Coord = (usize, usize);

fn find_trails(coord: Coord, map: &TopoMap) -> Vec<Vec<Coord>> {
    let (x, y) = coord;
    let altitude = map[y][x];

    if altitude == 9 {
        return vec![vec![coord]];
    }

    let mut paths = Vec::new();
    for xn in x.saturating_sub(1)..=usize::min(x + 1, map[0].len() - 1) {
        for yn in y.saturating_sub(1)..=usize::min(y + 1, map.len() - 1) {
            if xn != x && yn != y {
                continue;
            }

            let c = (xn, yn);

            if map[yn][xn] == altitude + 1 {
                let mut found = find_trails(c, map);
                for p in found.iter_mut() {
                    p.push(coord);
                    paths.push(p.clone())
                }
            }
        }
    }

    paths
}

fn solution(r: BufReader<File>) -> Result<(u64, u64), Box<dyn Error>> {
    let mut grid = Vec::<Vec<u8>>::new();
    let mut trailheads = Vec::<Coord>::new();
    let mut tops = HashSet::<Coord>::new();

    for (y, line) in r.lines().enumerate() {
        let line = line?;

        let mut grid_line = Vec::new();
        for (x, c) in line.char_indices() {
            let n = c.to_string().parse()?;
            grid_line.push(n);

            if n == 0 {
                trailheads.push((x, y));
            } else if n == 9 {
                tops.insert((x, y));
            }
        }

        grid.push(grid_line);
    }

    let part1 = trailheads
        .iter()
        .map(|th| {
            find_trails(*th, &grid)
                .iter()
                .map(|trail| trail[0])
                .collect::<HashSet<Coord>>()
        })
        .flatten()
        .count()
        .try_into()?;
    let part2 = trailheads
        .iter()
        .map(|th| find_trails(*th, &grid).len() as u64)
        .sum();

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

    assert_eq!(solution(r).unwrap(), (36, 81))
}

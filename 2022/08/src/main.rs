use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input.txt")?;
    let r = BufReader::new(f);

    let mut forest = Vec::<Vec<i8>>::new();

    for line in r.lines() {
        let line = line?;
        forest.push(
            line.chars()
                .map(|c| format!("{}", c).as_str().parse::<i8>().unwrap())
                .collect::<Vec<i8>>(),
        );
    }

    let width = forest.len();

    let mut visibility_top = forest.clone();
    for x in 0..width {
        for y in 0..width {
            if y == 0 {
                visibility_top[y][x] = -1;
            } else {
                visibility_top[y][x] = visibility_top[y - 1][x].max(forest[y - 1][x]);
            }
        }
    }

    let mut visibility_bot = forest.clone();
    for x in 0..width {
        for y in (0..width).rev() {
            if y == width - 1 {
                visibility_bot[y][x] = -1;
            } else {
                visibility_bot[y][x] = visibility_bot[y + 1][x].max(forest[y + 1][x]);
            }
        }
    }

    let mut visibility_left = forest.clone();
    for y in 0..width {
        for x in 0..width {
            if x == 0 {
                visibility_left[y][x] = -1;
            } else {
                visibility_left[y][x] = visibility_left[y][x - 1].max(forest[y][x - 1]);
            }
        }
    }

    let mut visibility_right = forest.clone();
    for y in 0..width {
        for x in (0..width).rev() {
            if x == width - 1 {
                visibility_right[y][x] = -1;
            } else {
                visibility_right[y][x] = visibility_right[y][x + 1].max(forest[y][x + 1]);
            }
        }
    }

    let mut visible = 0;
    for x in 0..width {
        for y in 0..width {
            let min_size = [
                visibility_bot[y][x],
                visibility_left[y][x],
                visibility_right[y][x],
                visibility_top[y][x],
            ]
            .into_iter()
            .min()
            .unwrap();
            if forest[y][x] > min_size {
                visible += 1;
            }
        }
    }

    println!("There are {} visible trees", visible);

    let mut max_scenic = 0;
    for x in 0..width {
        for y in 0..width {
            let size = forest[y][x];

            let mut up = 0;
            for a in (0..y).rev() {
                up += 1;
                if forest[a][x] >= size {
                    break;
                }
            }

            let mut down = 0;
            for a in y+1..width {
                down += 1;
                if forest[a][x] >= size {
                    break;
                }
            }

            let mut left = 0;
            for a in (0..x).rev() {
                left += 1;
                if forest[y][a] >= size {
                    break;
                }
            }

            let mut right = 0;
            for a in x+1..width {
                right += 1;
                if forest[y][a] >= size {
                    break;
                }
            }

            let score = up * down * left * right;
            max_scenic = max_scenic.max(score);
        }
    }

    println!("The highest scenic score is {}", max_scenic);

    Ok(())
}

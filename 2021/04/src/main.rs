use core::num;
use std::{collections::HashMap, error::Error, usize};

#[derive(Debug, Default, Clone, Copy)]
struct BingoField {
    number: usize,
    marked: bool,
}

#[derive(Debug, Clone)]
struct BingoBoard {
    board: Vec<Vec<BingoField>>,
    number_coords: HashMap<usize, (usize, usize)>,
    marked: Vec<(usize, usize)>,
}

impl BingoBoard {
    fn mark(&mut self, number: &usize) {
        if let Some(&(x, y)) = self.number_coords.get(number) {
            if *number != self.board[x][y].number {
                panic!("Marked wrong number");
            }
            self.board[x][y].marked = true;
            self.marked.push((x, y));
        }
    }

    fn has_won(&self) -> bool {
        let mut marked_cols = [0, 0, 0, 0, 0];
        let mut marked_rows = [0, 0, 0, 0, 0];

        for &(x, y) in self.marked.iter() {
            marked_cols[x] += 1;
            marked_rows[y] += 1;
        }

        marked_rows.iter().any(|&x| x == 5) || marked_cols.iter().any(|&x| x == 5)
    }

    fn score(&self, n: usize) -> usize {
        n * self.board.iter().fold(0usize, |score, col| {
            score
                + col.iter().fold(0usize, |score, &field| {
                    if field.marked {
                        score
                    } else {
                        score + field.number
                    }
                })
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    //let lines = include_str!("../example_input.txt").lines().collect::<Vec<&str>>();
    let lines = include_str!("../input.txt").lines().collect::<Vec<&str>>();

    let numbers = lines[0]
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut boards = Vec::<BingoBoard>::new();
    let board_strings = &lines[2..];

    for i in (5..).step_by(6) {
        if let None = board_strings.get(i - 1) {
            break;
        }

        let board: Vec<Vec<BingoField>> = board_strings[i - 5..i]
            .iter()
            .map(|&s| {
                s.split_whitespace()
                    .map(|n| BingoField {
                        number: n.parse().unwrap(),
                        marked: false,
                    })
                    .collect()
            })
            .collect();

        let mut number_coords = HashMap::new();
        for j in (0..5) {
            for k in (0..5) {
                number_coords.insert(board[j][k].number, (j, k));
            }
        }

        boards.push(BingoBoard {
            board,
            number_coords,
            marked: Vec::new(),
        })
    }

    for n in numbers {
        for board in boards.iter_mut() {
            if board.has_won() {
                continue;
            }

            board.mark(&n);

            if board.has_won() {
                println!("Score: {}", board.score(n));
            }
        }
    }

    Ok(())
}

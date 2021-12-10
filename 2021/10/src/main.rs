#[derive(Debug)]
enum Error {
    Incomplete(Vec<char>),
    Corruption(char),
}

fn main() {
    let lines: Vec<&str> = include_str!("../input.txt").lines().collect();
    //let lines: Vec<&str> = include_str!("../example_input.txt").lines().collect();

    let errors = lines
        .iter()
        .map(|&s| {
            let mut stack = Vec::<char>::new();
            for c in s.chars() {
                match c {
                    '(' => stack.push(')'),
                    '{' => stack.push('}'),
                    '[' => stack.push(']'),
                    '<' => stack.push('>'),
                    ')' | '}' | ']' | '>' => {
                        if stack.pop() != Some(c) {
                            return Error::Corruption(c);
                        }
                    }
                    _ => unreachable!(),
                }
            }

            Error::Incomplete(stack)
        })
        .collect::<Vec<Error>>();

    let score = errors.iter().fold(0, |score, e| {
        score
            + match e {
                Error::Corruption(')') => 3,
                Error::Corruption(']') => 57,
                Error::Corruption('}') => 1197,
                Error::Corruption('>') => 25137,
                _ => 0,
            }
    });
    dbg!(score);

    let mut scores = errors
        .iter()
        .map(|e| {
            if let Error::Incomplete(stack) = e {
                stack.into_iter().rev().fold(0u128, |score, c| {
                    score * 5
                        + match c {
                            ')' => 1,
                            ']' => 2,
                            '}' => 3,
                            '>' => 4,
                            _ => unreachable!(),
                        }
                })
            } else {
                0
            }
        })
        .filter(|&n| n > 0)
        .collect::<Vec<_>>();

    scores.sort();
    let score2 = scores[(scores.len() - 1) / 2];
    dbg!(score2);
}

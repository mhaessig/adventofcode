use std::{borrow::Borrow, collections::HashMap};

fn sub (s1: &str, s2: &str) -> String {
    let mut s = String::from(s1);
    for c in s2.chars() {
        s = s.replace(c, "");
    }

    s
}

fn main() {
    let wirings_numbers = include_str!("../input.txt")
    //let wirings_numbers = include_str!("../example_input.txt")
        .lines()
        .map(|s| s.split_once(" | ").unwrap())
        .map(|(w, n)| {
            (
                w.split_whitespace().collect::<Vec<_>>(),
                n.split_whitespace().collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let count = wirings_numbers.iter().fold(0, |count, (_, numbers)| {
        count
            + numbers.iter().fold(0, |count, &s| {
                if s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7 {
                    count + 1
                } else {
                    count
                }
            })
    });

    dbg!(count);

    let mut sum = 0;
    for (wiring, numbers) in wirings_numbers {
        let &one = wiring.iter().find(|&&s| s.len() == 2).unwrap();
        let &seven = wiring.iter().find(|&s| s.len() == 3).unwrap();
        let &four = wiring.iter().find(|&s| s.len() == 4).unwrap();
        let &eight = wiring.iter().find(|&s| s.len() == 7).unwrap();
        let rest = wiring
            .iter()
            .filter(|&&s| s.len() > 4 && s.len() < 7)
            .collect::<Vec<_>>();

        let top = sub(seven, one);
        let top_left_mid = sub(four, one);
        let bot_left_bot = sub(&sub(eight, &top_left_mid), seven);
        let bot = rest
            .iter()
            .map(|&&s| sub(&sub(s, seven), &top_left_mid))
            .filter(|s| s.len() == 1)
            .collect::<Vec<_>>()[0].clone();
        let bot_left = sub(&bot_left_bot, &bot);
        let mid = wiring
            .iter()
            .map(|&s| sub(&sub(s, seven), &bot))
            .filter(|s| s.len() == 1)
            .collect::<Vec<_>>()[0].clone();
        let top_left = sub(&top_left_mid, &mid);
        let bot_right = rest
            .iter()
            .filter(|&&&s| s.len() == 6)
            .map(|&&s| sub(&sub(&sub(&s, &bot_left_bot), &top_left_mid), &top))
            .filter(|s| s.len() == 1)
            .collect::<Vec<_>>()[0].clone();
        let top_right = sub(one, &bot_right);

        let mut number_map = HashMap::<String, usize>::new();

        number_map.insert(
            to_sorted_str(&vec![top_right.clone(), bot_right.clone(), top.clone(), bot.clone(), top_left.clone(), bot_left.clone()]),
            0,
        );
        number_map.insert(to_sorted_str(&vec![top_right.clone(), bot_right.clone()]), 1);
        number_map.insert(to_sorted_str(&vec![top.clone(), top_right.clone(), mid.clone(), bot_left.clone(), bot.clone()]), 2);
        number_map.insert(to_sorted_str(&vec![top_right.clone(), bot_right.clone(), top.clone(), mid.clone(), bot.clone()]), 3);
        number_map.insert(to_sorted_str(&vec![top_right.clone(), bot_right.clone(), top_left.clone(), mid.clone()]), 4);
        number_map.insert(to_sorted_str(&vec![top_left.clone(), bot_right.clone(), top.clone(), mid.clone(), bot.clone()]), 5);
        number_map.insert(
            to_sorted_str(&vec![top_left.clone(), bot_left.clone(), bot_right.clone(), top.clone(), mid.clone(), bot.clone()]),
            6,
        );
        number_map.insert(to_sorted_str(&vec![top_right.clone(), bot_right.clone(), top.clone()]), 7);
        number_map.insert(
            to_sorted_str(&vec![
                top_right.clone(), bot_right.clone(), top_left.clone(), bot_left.clone(), top.clone(), mid.clone(), bot.clone(),
            ]),
            8,
        );
        number_map.insert(
            to_sorted_str(&vec![top_right.clone(), bot_right.clone(), top_left.clone(), top.clone(), mid.clone(), bot.clone()]),
            9,
        );

        let res = numbers.iter().rev().enumerate().fold(0, |n, (exp, &s)| {
            let mut sorted: Vec<char> = s.chars().collect();
            sorted.sort();
            n + number_map
                .get(
                    &sorted
                        .into_iter()
                        .fold(String::new(), |s, c| format!("{}{}", s, c)))
                .unwrap()
                .to_owned()
                * 10usize.pow(exp as u32)
        });
        sum += res;
    }

    dbg!(sum);
}

fn to_sorted_str<'a>(slc: &Vec<String>) -> String {
    let mut s = slc.clone();
    s.sort();
    s.iter().fold(String::new(), |s, c| format!("{}{}", &s, &c))
}

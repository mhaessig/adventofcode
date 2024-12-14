use itertools::{Itertools, MinMaxResult};

struct Rule {
    pair: String,
    new_thing: String,
}

fn main() {
    let mut polymer = String::new();
    let mut rules: Vec<Rule> = Vec::new();

    //let lines = include_str!("../input.txt").lines();
    let lines = include_str!("../example_input.txt").lines();

    let mut empty_line_found = false;
    for l in lines {
        if !empty_line_found {
            if l.is_empty() {
                empty_line_found = true;
                continue;
            }

            polymer = String::from(l);
        } else {
            let (pair, element) = l.split_once(" -> ").unwrap();
            rules.push(Rule {
                pair: String::from(pair),
                new_thing: format!("{}{}{}", &pair[0..1], &element, &pair[1..2]),
            });
        }
    }

    for j in 0..40 {
        println!("{}", j);
        for r in rules.iter() {
            polymer = polymer.replace(r.pair.as_str(), r.new_thing.as_str());
        }
    }

    match polymer.chars().counts().values().minmax() {
        MinMaxResult::MinMax(min, max) => println!("max - min = {} - {} = {}", max, min, max - min),
        _ => unreachable!(),
    }
}

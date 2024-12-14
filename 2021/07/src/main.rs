fn main() {
    //let positions: Vec<usize> = include_str!("../example_input.txt")
    let positions: Vec<usize> = include_str!("../input.txt")
        .lines()
        .flat_map(|s| s.split(',').collect::<Vec<_>>())
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let &min_pos = positions.iter().min().unwrap();
    let &max_pos = positions.iter().max().unwrap();

    let mut optimum = usize::MAX;
    let mut best_cost = usize::MAX;
    (min_pos..=max_pos).for_each(|i| {
        let cost = positions.iter().fold(0, |sum, &pos| diff(pos, i) + sum);
        if cost < best_cost {
            best_cost = cost;
            optimum = i
        }
    });

    println!("Optimum position {} with cost  {}", optimum, best_cost);

    let mut sq_optimum = usize::MAX;
    let mut sq_best_cost = usize::MAX;
    (min_pos..=max_pos).for_each(|i| {
        let cost = positions.iter().fold(0, |sum, &pos| {
            let diff = diff(pos, i);
            (diff * (diff + 1) / 2) + sum
        });
        if cost < sq_best_cost {
            sq_best_cost = cost;
            sq_optimum = i
        }
    });

    println!("Optimum position {} with cost {}", sq_optimum, sq_best_cost);
}

fn diff(n1: usize, n2: usize) -> usize {
    if n1 < n2 {
        n2 - n1
    } else {
        n1 - n2
    }
}

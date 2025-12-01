fn main() {
    let map: Vec<Vec<u32>> = include_str!("../input.txt")
    //let map: Vec<Vec<u32>> = include_str!("../example_input.txt")
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let imax = map.len() - 1;
    let jmax = map[0].len() - 1;
    let mut lowpoints = vec![];
    for (i, row) in map.iter().enumerate() {
        for (j, &n) in row.iter().enumerate() {
            let mut low = true;
            if i > 0 {
                low = low && map[i - 1][j] > n;
            }
            if i < imax {
                low = low && map[i + 1][j] > n;
            }
            if j > 0 {
                low = low && map[i][j - 1] > n;
            }
            if j < jmax {
                low = low && map[i][j + 1] > n;
            }

            if low {
                lowpoints.push((i, j));
            }
        }
    }

    let sum = lowpoints
        .iter()
        .fold(0, |sum, (i, j)| sum + 1 + map[*i][*j]);
    dbg!(sum);

    let mut sizes = Vec::new();
    for p in lowpoints {
        let mut basin: Vec<(usize, usize)> = vec![p];

        let mut found_more = true;
        while found_more {
            found_more = false;

            for (i, j) in basin.clone() {
                if i > 0 {
                    let new_p = (i - 1, j);
                    if map[new_p.0][new_p.1] != 9 && !basin.contains(&new_p) {
                        basin.push(new_p);
                        found_more = true;
                    }
                }
                if i < imax {
                    let new_p = (i + 1, j);
                    if map[new_p.0][new_p.1] != 9 && !basin.contains(&new_p) {
                        basin.push(new_p);
                        found_more = true;
                    }
                }
                if j > 0 {
                    let new_p = (i, j - 1);
                    if map[new_p.0][new_p.1] != 9 && !basin.contains(&new_p) {
                        basin.push(new_p);
                        found_more = true;
                    }
                }
                if j < jmax {
                    let new_p = (i, j + 1);
                    if map[new_p.0][new_p.1] != 9 && !basin.contains(&new_p) {
                        basin.push(new_p);
                        found_more = true;
                    }
                }
            }
        }
        sizes.push(basin.len());
    }

    sizes.sort();
    sizes.reverse();
    let prod = sizes[0..3].iter().product::<usize>();
    dbg!(prod);
}

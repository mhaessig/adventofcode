use std::borrow::BorrowMut;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Sum {
    sum: u128,
    numbers: (u128, u128),
}

struct SumTree {
    idx: usize,
    sum: u128,
    low_elem: u128,
    high_elem: u128,
    root_idx: usize,
}
fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input.txt")?;
    let r = BufReader::new(f);

    let mut wrong = 0;
    let mut idx = 0;
    let mut number_list = Vec::<u128>::new();
    let mut numbers = Vec::<u128>::new();
    let mut sums = Vec::<Sum>::new();
    for (i, l) in r.lines().enumerate() {
        let n = l?.as_str().parse::<u128>()?;
        number_list.push(n);

        if i < 25 {
            sums.append(
                numbers
                    .clone()
                    .iter()
                    .map(|elem| Sum {
                        sum: elem + n,
                        numbers: (*elem, n),
                    })
                    .collect::<Vec<Sum>>()
                    .borrow_mut(),
            );
            numbers.push(n);
        } else {
            if !sums.iter().any(|sum| sum.sum == n) {
                wrong = n;
                idx = i;
                break;
            }

            let removed = numbers.remove(0);
            for _ in 0..24 {
                sums.retain(|sum| sum.numbers.0 != removed && sum.numbers.1 != removed);
            }
            sums.append(
                numbers
                    .clone()
                    .iter()
                    .map(|elem| Sum {
                        sum: elem + n,
                        numbers: (*elem, n),
                    })
                    .collect::<Vec<Sum>>()
                    .borrow_mut(),
            );
            numbers.push(n);
        }
    }

    println!("Part 1: first wrong number: {} (i={})", wrong, idx);

    let mut start_list = Vec::<SumTree>::new();
    let mut vuln = 0;
    for (i, n) in number_list.iter().enumerate() {
        for node in start_list.iter_mut().filter(|tr| tr.idx + 1 == i) {
            node.idx = i;
            node.sum += *n;
            node.low_elem = node.low_elem.min(*n);
            node.high_elem = node.high_elem.max(*n);

            if node.sum == wrong {
                vuln = node.low_elem + node.high_elem;
                break;
            }
        }

        let start = SumTree {
            idx: i,
            sum: *n,
            low_elem: *n,
            high_elem: *n,
            root_idx: i,
        };
        start_list.push(start);
    }

    println!("Part 2: weakness is {:?}", vuln);

    Ok(())
}

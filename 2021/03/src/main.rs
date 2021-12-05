#[derive(Debug, Clone, Copy)]
struct Bitcount {
    zeros: usize,
    ones: usize,
}

impl Bitcount {
    fn new() -> Self {
        Bitcount { zeros: 0, ones: 0 }
    }
}

#[derive(Clone, Copy)]
enum RatingType {
    // most common
    OxygenGenerator,
    // least common
    CO2Scrubber,
}

impl RatingType {
    fn get_bit_to_keep(self, bitcount: &Bitcount) -> u32 {
        match self {
            Self::OxygenGenerator => if bitcount.zeros <= bitcount.ones {1} else {0},
            Self::CO2Scrubber => if bitcount.zeros <= bitcount.ones {0} else {1}
        }
    }
}

fn main() {
    //let bits = include_str!("../example_input.txt")
    let bits = include_str!("../input.txt")
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let count = get_bitcount(&bits);

    let gamma = count
        .iter()
        .rev()
        .enumerate()
        .fold(0, |gamma, (exp, bitcount)| {
            if bitcount.zeros > bitcount.ones {
                gamma
            } else if bitcount.zeros < bitcount.ones {
                gamma + 2usize.pow(exp.try_into().unwrap())
            } else {
                panic!("equal bitcount")
            }
        });

    let epsilon = count
        .iter()
        .rev()
        .enumerate()
        .fold(0, |gamma, (exp, bitcount)| {
            if bitcount.zeros < bitcount.ones {
                gamma
            } else if bitcount.zeros > bitcount.ones {
                gamma + 2usize.pow(exp.try_into().unwrap())
            } else {
                panic!("equal bitcount")
            }
        });

    dbg!(gamma * epsilon);

    let oxygen_rating = find_rating(&bits, &count, RatingType::OxygenGenerator);
    dbg!(oxygen_rating);
    let co2_rating = find_rating(&bits, &count, RatingType::CO2Scrubber);
    let life_support_rating = oxygen_rating * co2_rating;

    dbg!(co2_rating);
    dbg!(life_support_rating);

}

fn find_rating(bits: &Vec<Vec<u32>>, count: &Vec<Bitcount>, rating_type: RatingType) -> u32 {
    let mut numbers = bits.clone();
    let mut bitcount = count.clone().to_owned();
    let mut bit_idx: usize = 0;
    while numbers.len() != 1 {
        let bitcount_at_idx = &bitcount[bit_idx].to_owned();
        let keep_bit_at_idx = rating_type.get_bit_to_keep(&bitcount_at_idx);

        
        numbers = numbers
            .into_iter()
            .filter(|num| num[bit_idx] == keep_bit_at_idx)
            .collect();

        bitcount = get_bitcount(&numbers);

        bit_idx += 1;
    }

    let number = numbers
        .into_iter()
        .flatten()
        .collect::<Vec<u32>>();

    dbg!(&number);
    
    number.into_iter().rev().enumerate().fold(0u32, |n, (exp, bit)| {
            n + bit * 2u32.pow(exp.try_into().unwrap())
        })
}

fn get_bitcount(bits: &Vec<Vec<u32>>) -> Vec<Bitcount> {
    let mut count = Vec::<Bitcount>::new();
    for (i, n) in bits.iter().enumerate() {
        for (j, bit) in n.iter().enumerate() {
            if i == 0 {
                count.push(Bitcount::new());
            }

            match bit {
                0 => count[j].zeros += 1,
                1 => count[j].ones += 1,
                _ => panic!("not a bit!"),
            }
        }
    }

    count
}

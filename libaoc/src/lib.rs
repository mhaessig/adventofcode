use std::{error::Error, fmt::Debug, str::FromStr};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn parse_vec<T>(input: String) -> Result<Vec<T>, Box<dyn Error>>
where
    T: FromStr + Debug,
    T: FromStr<Err = Box<dyn std::error::Error + 'static>> + Debug
{
    input
            .lines()
            .map(T::from_str)
            .map(|elem| elem)
            .collect::<Result<Vec<T>, _>>()
}

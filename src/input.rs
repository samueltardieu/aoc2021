use anyhow::Result;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

pub fn input_bytes(day: usize) -> Result<Vec<u8>> {
    let filename = match { unsafe { &super::OVERRIDE_INPUT } } {
        Some(s) => s.clone(),
        None => format!("input/day{}.txt", day),
    };
    let mut res = Vec::new();
    File::open(filename)?.read_to_end(&mut res)?;
    Ok(res)
}

pub fn input_string(day: usize) -> Result<String> {
    Ok(String::from_utf8(input_bytes(day)?)?)
}

pub fn parse_lines<T>(input: &str) -> Result<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    Ok(input
        .lines()
        .map(T::from_str)
        .collect::<std::result::Result<Vec<T>, _>>()?)
}

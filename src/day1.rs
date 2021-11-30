use std::num::ParseIntError;

#[aoc_generator(day1)]
fn generator(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.lines().map(|s| s.parse()).collect()
}

#[aoc(day1, part1)]
fn part1(input: &[u32]) -> u32 {
    0
}

#[aoc(day1, part2)]
fn part2(input: &[u32]) -> u32 {
    0
}
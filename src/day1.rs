use itertools::Itertools;
use std::num::ParseIntError;

#[aoc_generator(day1)]
fn generator(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.lines().map(|s| s.parse()).collect()
}

#[aoc(day1, part1)]
fn part1(depths: &[u32]) -> usize {
    depths.iter().tuple_windows().filter(|(a, b)| b > a).count()
}

#[aoc(day1, part2)]
fn part2(depths: &[u32]) -> usize {
    let depths = depths
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .collect_vec();
    part1(&depths)
}

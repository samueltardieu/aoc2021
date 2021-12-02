use std::str::FromStr;

use aoc2021::input;
use aoc2021_derive::aoc;

#[aoc(day1, part1)]
fn day1_part1() -> usize {
    30
}

#[aoc(day1, part2)]
fn day1_part2() -> anyhow::Result<usize> {
    Ok(40)
}

#[aoc(day1, part2, alternate)]
fn day1_part2_alternate() -> anyhow::Result<usize> {
    Ok(50)
}

#[test]
fn result() {
    assert_eq!(30, day1_part1());
    assert_eq!(40, day1_part2().unwrap());
    assert_eq!(50, day1_part2_alternate().unwrap());
}

#[test]
fn runner() {
    assert_eq!(30, runner_1_1_none().unwrap());
    assert_eq!(40, runner_1_2_none().unwrap());
    assert_eq!(50, runner_1_2_alternate().unwrap());
}

fn set_test_input() {
    unsafe {
        aoc2021::OVERRIDE_INPUT = Some(String::from("tests/input.txt"));
    }
}

#[aoc(day3, part1, str)]
fn d3p1s(input: &str) -> String {
    input.replace('\n', "")
}

#[aoc(day3, part1, bytes)]
fn d3p1b(input: &[u8]) -> anyhow::Result<String> {
    Ok(d3p1s(std::str::from_utf8(input)?))
}

#[aoc(day3, part1, u32)]
fn d3p1u32(input: &[u32]) -> u32 {
    input.iter().sum()
}

#[aoc(day3, part1, s)]
fn d1p1ss(input: &[S]) -> u8 {
    input.iter().map(|s| s.u).sum()
}

struct S {
    u: u8,
}

impl FromStr for S {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(S {
            u: s.as_bytes()[0] - b'0',
        })
    }
}

#[test]
fn inputs() {
    set_test_input();
    assert_eq!("123102030", runner_3_1_str().unwrap());
    assert_eq!("123102030", runner_3_1_bytes().unwrap());
    assert_eq!(66, runner_3_1_u32().unwrap());
    assert_eq!(12, runner_3_1_s().unwrap());
}

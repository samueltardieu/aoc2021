#[macro_use]
extern crate aoc2021_derive;

pub static mut OVERRIDE_INPUT: Option<String> = None;

mod input;
pub mod runners;

pub mod day1;
pub mod day2;

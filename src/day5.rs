use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{char, i16},
    sequence::separated_pair,
    IResult,
};
use std::{collections::HashMap, iter, str::FromStr};

type Point = (i16, i16);
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_straight(&self) -> bool {
        self.start.0 == self.end.0 || self.start.1 == self.end.1
    }

    fn points(&self) -> impl Iterator<Item = Point> {
        let delta = (
            (self.end.0 - self.start.0).signum(),
            (self.end.1 - self.start.1).signum(),
        );
        let end = self.end;
        iter::successors(Some(self.start), move |p| {
            Some((p.0 + delta.0, p.1 + delta.1))
        })
        .take_while(move |p| p != &end)
        .chain(iter::once(self.end))
    }
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    separated_pair(i16, char(','), i16)(input)
}

impl FromStr for Line {
    type Err = nom::Err<nom::error::Error<String>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        separated_pair(parse_point, tag(" -> "), parse_point)(s)
            .map(|(_, (start, end))| Line { start, end })
            .map_err(|e| e.to_owned())
    }
}

fn parse(input: &str) -> impl Iterator<Item = Line> + '_ {
    input.lines().map(|l| l.parse().unwrap())
}

#[aoc(day5, part1)]
fn part1(input: &str) -> Result<usize> {
    let mut points = HashMap::new();
    parse(input)
        .filter(Line::is_straight)
        .flat_map(|l| l.points())
        .for_each(|p| *points.entry(p).or_insert(0) += 1);
    Ok(points.values().filter(|v| **v > 1).count())
}

#[aoc(day5, part2)]
fn part2(input: &str) -> Result<usize> {
    let mut points = HashMap::new();
    parse(input)
        .flat_map(|l| l.points())
        .for_each(|p| *points.entry(p).or_insert(0) += 1);
    Ok(points.values().filter(|v| **v > 1).count())
}

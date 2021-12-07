use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{char, i16},
    error::Error,
    sequence::separated_pair,
    Finish, IResult,
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
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match separated_pair(parse_point, tag(" -> "), parse_point)(s).finish() {
            Ok((_, (start, end))) => Ok(Line { start, end }),
            Err(Error { input, code }) => Err(Error::new(input.to_owned(), code)),
        }
    }
}

#[aoc(day5, part1)]
fn part1(input: Vec<Line>) -> Result<usize> {
    let mut points = HashMap::new();
    input
        .into_iter()
        .filter(Line::is_straight)
        .flat_map(|l| l.points())
        .for_each(|p| *points.entry(p).or_insert(0) += 1);
    Ok(points.values().filter(|v| **v > 1).count())
}

#[aoc(day5, part2)]
fn part2(input: Vec<Line>) -> Result<usize> {
    let mut points = HashMap::new();
    input
        .into_iter()
        .flat_map(|l| l.points())
        .for_each(|p| *points.entry(p).or_insert(0) += 1);
    Ok(points.values().filter(|v| **v > 1).count())
}

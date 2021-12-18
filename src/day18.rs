use itertools::iproduct;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    sequence::{delimited, separated_pair},
    Finish, IResult,
};
use std::{
    ops::{Add, ControlFlow},
    str::FromStr,
};

#[derive(Debug)]
enum SnailFish {
    Regular(u32),
    Pair(Box<SnailFish>, Box<SnailFish>),
}

impl SnailFish {
    fn value(&self) -> Option<u32> {
        match self {
            SnailFish::Regular(n) => Some(*n),
            SnailFish::Pair(..) => None,
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            SnailFish::Regular(n) => *n,
            SnailFish::Pair(a, b) => 3 * a.magnitude() + 2 * b.magnitude(),
        }
    }

    fn reduce(mut self) -> Self {
        loop {
            while let ControlFlow::Break((i, a, b)) = self.check_explode(0, 0) {
                self.do_explode(0, &(i, a, b));
            }
            if !self.split() {
                return self;
            }
        }
    }

    fn check_explode(
        &mut self,
        index: usize,
        depth: usize,
    ) -> ControlFlow<(usize, u32, u32), usize> {
        match self {
            SnailFish::Pair(a, b) if depth == 4 => {
                let r = ControlFlow::Break((index, a.value().unwrap(), b.value().unwrap()));
                *self = SnailFish::Regular(0);
                r
            }
            SnailFish::Pair(a, b) => {
                let index = a.check_explode(index, depth + 1)?;
                let index = b.check_explode(index, depth + 1)?;
                ControlFlow::Continue(index)
            }
            SnailFish::Regular(_) => ControlFlow::Continue(index + 1),
        }
    }

    fn do_explode(
        &mut self,
        index: usize,
        e @ (i, a, b): &(usize, u32, u32),
    ) -> ControlFlow<(), usize> {
        match self {
            SnailFish::Regular(n) if index + 1 == *i => {
                *self = SnailFish::Regular(*n + a);
                ControlFlow::Continue(index + 1)
            }
            SnailFish::Regular(n) if index == *i + 1 => {
                *self = SnailFish::Regular(*n + b);
                ControlFlow::Break(())
            }
            SnailFish::Regular(_) => ControlFlow::Continue(index + 1),
            SnailFish::Pair(a, b) => {
                let index = a.do_explode(index, e)?;
                let index = b.do_explode(index, e)?;
                ControlFlow::Continue(index)
            }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            SnailFish::Regular(n) if *n >= 10 => {
                *self = SnailFish::Regular(*n / 2) + SnailFish::Regular((*n + 1) / 2);
                true
            }
            SnailFish::Regular(_) => false,
            SnailFish::Pair(a, b) => a.split() || b.split(),
        }
    }
}

impl Add<SnailFish> for SnailFish {
    type Output = SnailFish;

    fn add(self, rhs: Self) -> Self::Output {
        SnailFish::Pair(Box::new(self), Box::new(rhs))
    }
}

impl Clone for SnailFish {
    fn clone(&self) -> Self {
        match self {
            SnailFish::Regular(n) => SnailFish::Regular(*n),
            SnailFish::Pair(a, b) => a.as_ref().clone() + b.as_ref().clone(),
        }
    }
}

impl FromStr for SnailFish {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s)
            .finish()
            .map(|(_, f)| f)
            .map_err(|e| nom::error::Error::new(e.input.to_owned(), e.code))
    }
}

fn parse(line: &str) -> IResult<&str, SnailFish> {
    alt((
        map(nom::character::complete::u32, SnailFish::Regular),
        delimited(
            tag("["),
            map(separated_pair(parse, tag(","), parse), |(a, b)| a + b),
            tag("]"),
        ),
    ))(line)
}

#[aoc(day18, part1)]
fn part1(trees: Vec<SnailFish>) -> u32 {
    trees
        .into_iter()
        .reduce(|a, b| (a + b).reduce())
        .unwrap()
        .magnitude()
}

#[aoc(day18, part2)]
fn part2(trees: &[SnailFish]) -> u32 {
    iproduct!(0..trees.len(), 0..trees.len())
        .filter_map(|(a, b)| {
            (a != b).then(|| (trees[a].clone() + trees[b].clone()).reduce().magnitude())
        })
        .max()
        .unwrap()
}

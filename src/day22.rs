use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    Finish, IResult,
};
use std::str::FromStr;

#[derive(Clone, Debug)]
struct Cuboid {
    lit: bool,
    area: Vec<(i64, i64)>,
}

impl Cuboid {
    fn parse(line: &str) -> IResult<&str, Self> {
        use nom::character::complete::i64;
        let lit = |i| alt((map(tag("on "), |_| true), map(tag("off "), |_| false)))(i);
        let coords = |i| preceded(take(2usize), separated_pair(i64, tag(".."), i64))(i);
        let area = move |i| separated_list1(tag(","), coords)(i);
        map(tuple((lit, area)), |(lit, area)| Cuboid { lit, area })(line)
    }

    fn intersect(&self, other: &Self) -> Option<Cuboid> {
        Some(Cuboid {
            lit: !self.lit,
            area: self
                .area
                .iter()
                .zip(&other.area)
                .map(|(&(a1, a2), &(b1, b2))| {
                    let (c1, c2) = (a1.max(b1), a2.min(b2));
                    (c1 <= c2).then(|| (c1, c2))
                })
                .collect::<Option<_>>()?,
        })
    }

    fn volume(&self) -> i64 {
        let v = self.area.iter().map(|&(a, b)| b - a + 1).product::<i64>();
        [-v, v][self.lit as usize]
    }
}

impl FromStr for Cuboid {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
            .finish()
            .map(|(_, b)| b)
            .map_err(|nom::error::Error { input, code }| nom::error::Error {
                input: input.to_owned(),
                code,
            })
    }
}

#[aoc(day22, part1)]
fn part1(mut cuboids: Vec<Cuboid>) -> i64 {
    cuboids.retain(|b| b.area.iter().all(|c| c.0 >= -50 && c.1 <= 50));
    complete(cuboids)
}

#[aoc(day22, part2)]
fn part2(cuboids: Vec<Cuboid>) -> i64 {
    complete(cuboids)
}

fn complete(cuboids: Vec<Cuboid>) -> i64 {
    let mut c = Vec::<Cuboid>::new();
    for cuboid in cuboids {
        for other in 0..c.len() {
            c.extend(c[other].intersect(&cuboid));
        }
        c.extend(cuboid.lit.then(|| cuboid));
    }
    c.iter().map(Cuboid::volume).sum()
}

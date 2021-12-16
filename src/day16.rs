use nom::{bits::complete as bits, multi, sequence, Finish, IResult};
use std::str::FromStr;

enum Packet {
    Literal(u8, u64),
    Operator(u8, u8, Vec<Packet>),
}

impl Packet {
    fn parse(input: (&[u8], usize)) -> IResult<(&[u8], usize), Self> {
        match sequence::tuple((bits::take(3usize), bits::take(3usize)))(input)? {
            (input, (version, 4u8)) => Self::parse_number(input, version),
            (input, (version, op)) => Self::parse_operator(input, version, op),
        }
    }

    fn parse_number(input: (&[u8], usize), version: u8) -> IResult<(&[u8], usize), Self> {
        let (input, n) = multi::fold_many0(
            sequence::tuple((bits::tag(1, 1usize), bits::take(4usize))),
            || 0,
            |n, (_, x): (_, u64)| (n << 4) | x,
        )(input)?;
        let (input, (_, x)): (_, (_, u64)) =
            sequence::tuple((bits::tag(0, 1usize), bits::take(4usize)))(input)?;
        Ok((input, Packet::Literal(version, (n << 4) | x)))
    }

    fn parse_operator(input: (&[u8], usize), version: u8, op: u8) -> IResult<(&[u8], usize), Self> {
        let (input, packets) = match bits::take(1usize)(input)? {
            (input, 0) => {
                let (mut input, len): (_, usize) = bits::take(15usize)(input)?;
                let limit = input.0.len() * 8 - input.1 - len;
                let packets = std::iter::from_fn(|| {
                    (input.0.len() * 8 - input.1 > limit).then(|| {
                        let (rest, packet) = Self::parse(input)?;
                        input = rest;
                        Ok(packet)
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
                (input, packets)
            }
            (input, _) => {
                multi::length_count(bits::take::<_, usize, _, _>(11usize), Self::parse)(input)?
            }
        };
        Ok((input, Packet::Operator(version, op, packets)))
    }

    fn version_sum(&self) -> u32 {
        match &self {
            Packet::Literal(version, _) => *version as u32,
            Packet::Operator(version, _, sub) => {
                *version as u32 + sub.iter().map(Packet::version_sum).sum::<u32>()
            }
        }
    }

    fn eval(&self) -> u64 {
        match &self {
            Packet::Literal(_, n) => *n,
            Packet::Operator(_, 0, v) => v.iter().map(Packet::eval).sum(),
            Packet::Operator(_, 1, v) => v.iter().map(Packet::eval).product(),
            Packet::Operator(_, 2, v) => v.iter().map(Packet::eval).min().unwrap(),
            Packet::Operator(_, 3, v) => v.iter().map(Packet::eval).max().unwrap(),
            Packet::Operator(_, 5, v) => (v[0].eval() > v[1].eval()) as u64,
            Packet::Operator(_, 6, v) => (v[0].eval() < v[1].eval()) as u64,
            Packet::Operator(_, 7, v) => (v[0].eval() == v[1].eval()) as u64,
            _ => unreachable!(),
        }
    }
}

impl FromStr for Packet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = Vec::with_capacity((s.len() + 1) / 2);
        let mut chars = s.chars().fuse();
        while let Some(c) = chars.next() {
            data.push(
                ((c.to_digit(16).unwrap() << 4)
                    | chars.next().map(|c| c.to_digit(16).unwrap()).unwrap_or(0))
                    as u8,
            );
        }
        let r = nom::bits::bits(Packet::parse)(&data);
        r.finish()
            .map(|(_, p)| p)
            .map_err(|e: nom::error::Error<_>| anyhow::anyhow!("parsing error: {:?}", e))
    }
}

#[aoc(day16, part1)]
fn part1(input: &str) -> anyhow::Result<u32> {
    Ok(Packet::from_str(input.trim())?.version_sum())
}

#[aoc(day16, part2)]
fn part2(input: &str) -> anyhow::Result<u64> {
    Ok(Packet::from_str(input.trim())?.eval())
}

use nom::Finish;
use std::str::FromStr;

mod parser;

pub enum Packet {
    Literal(u8, u64),
    Operator(u8, u8, Vec<Packet>),
}

impl Packet {
    pub fn version_sum(&self) -> u32 {
        match &self {
            Packet::Literal(version, _) => *version as u32,
            Packet::Operator(version, _, sub) => {
                *version as u32 + sub.iter().map(Packet::version_sum).sum::<u32>()
            }
        }
    }

    pub fn eval(&self) -> u64 {
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

impl<'a> TryFrom<&'a [u8]> for Packet {
    type Error = nom::error::Error<&'a [u8]>;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        let r = nom::bits::bits(Packet::parse)(data);
        r.finish().map(|(_, p)| p)
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
        Self::try_from(data.as_ref())
            .map_err(|e: nom::error::Error<_>| anyhow::anyhow!("parsing error: {:?}", e))
    }
}

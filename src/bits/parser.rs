use nom::{bits::complete::*, combinator::*, multi::*, sequence::*, IResult};

use super::Packet;

impl Packet {
    pub(super) fn parse(input: (&[u8], usize)) -> IResult<(&[u8], usize), Self> {
        match tuple((take(3usize), take(3usize)))(input)? {
            (input, (version, 4u8)) => Self::parse_literal(input, version),
            (input, (version, op)) => Self::parse_operator(input, version, op),
        }
    }

    fn parse_literal(input: (&[u8], usize), version: u8) -> IResult<(&[u8], usize), Self> {
        let (input, n) = fold_many0(
            preceded(tag(1, 1usize), take(4usize)),
            || 0,
            |n, x: u64| (n | x) << 4,
        )(input)?;
        map(preceded(tag(0, 1usize), take(4usize)), move |x: u64| {
            Packet::Literal(version, n | x)
        })(input)
    }

    fn parse_operator(input: (&[u8], usize), version: u8, op: u8) -> IResult<(&[u8], usize), Self> {
        let (input, packets) = match take(1usize)(input)? {
            (input, 0) => {
                let (mut input, len): (_, usize) = take(15usize)(input)?;
                let limit = input.0.len() * 8 - input.1 - len;
                let packets = std::iter::from_fn(|| {
                    (input.0.len() * 8 - input.1 > limit).then(|| {
                        let (rest, packet) = Self::parse(input)?;
                        input = rest;
                        Ok(packet)
                    })
                })
                .collect::<Result<_, _>>()?;
                (input, packets)
            }
            (input, _) => length_count(take::<_, usize, _, _>(11usize), Self::parse)(input)?,
        };
        Ok((input, Packet::Operator(version, op, packets)))
    }
}

use std::{convert::Infallible, str::FromStr};

enum Packet {
    Literal(u8, u64),
    Operator(u8, u8, Vec<Packet>),
}

impl Packet {
    fn new(it: &mut DataIterator) -> Self {
        let version = it.bits(3) as u8;
        match it.bits(3) {
            4 => Self::new_number(version, it),
            type_id => Self::new_operator(version, it, type_id as u8),
        }
    }

    fn new_number(version: u8, it: &mut DataIterator) -> Self {
        let mut n = 0;
        while it.next().unwrap() {
            n = (n | it.bits(4)) << 4;
        }
        Packet::Literal(version, n | it.bits(4))
    }

    fn new_operator(version: u8, it: &mut DataIterator, type_id: u8) -> Self {
        let packets = if it.bits(1) == 0 {
            let next_index = it.bits(15) as usize + it.index;
            std::iter::from_fn(|| (it.index < next_index).then(|| Packet::new(it))).collect()
        } else {
            (0..(it.bits(11) as usize))
                .map(|_| Packet::new(it))
                .collect()
        };
        Packet::Operator(version, type_id, packets)
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

struct DataIterator {
    data: Vec<u8>,
    index: usize,
}

impl DataIterator {
    fn bits(&mut self, len: usize) -> u64 {
        (0..len).fold(0, |r, _| (r << 1) | self.next().unwrap() as u64)
    }
}

impl FromStr for DataIterator {
    type Err = Infallible;

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
        Ok(DataIterator { data, index: 0 })
    }
}

impl Iterator for DataIterator {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        (self.index < self.data.len() * 8).then(|| {
            let r = self.data[self.index / 8] & (0x80 >> (self.index % 8)) != 0;
            self.index += 1;
            r
        })
    }
}

#[aoc(day16, part1)]
fn part1(input: &str) -> u32 {
    Packet::new(&mut DataIterator::from_str(input.trim()).unwrap()).version_sum()
}

#[aoc(day16, part2)]
fn part2(input: &str) -> u64 {
    Packet::new(&mut DataIterator::from_str(input.trim()).unwrap()).eval()
}

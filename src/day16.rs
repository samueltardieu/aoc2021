use std::{convert::Infallible, str::FromStr};

struct Packet {
    version: u8,
    payload: Payload,
}

impl Packet {
    fn new(it: &mut DataIterator) -> Self {
        Packet {
            version: it.bits(3) as u8,
            payload: Payload::new(it),
        }
    }

    fn version_sum(&self) -> u32 {
        self.version as u32
            + match &self.payload {
                Payload::Literal(_) => 0,
                Payload::Operator(_, sub) => sub.iter().map(Packet::version_sum).sum(),
            }
    }

    fn eval(&self) -> u64 {
        match &self.payload {
            Payload::Literal(n) => *n,
            Payload::Operator(0, v) => v.iter().map(Packet::eval).sum(),
            Payload::Operator(1, v) => v.iter().map(Packet::eval).product(),
            Payload::Operator(2, v) => v.iter().map(Packet::eval).min().unwrap(),
            Payload::Operator(3, v) => v.iter().map(Packet::eval).max().unwrap(),
            Payload::Operator(5, v) => (v[0].eval() > v[1].eval()) as u64,
            Payload::Operator(6, v) => (v[0].eval() < v[1].eval()) as u64,
            Payload::Operator(7, v) => (v[0].eval() == v[1].eval()) as u64,
            _ => unreachable!(),
        }
    }
}

enum Payload {
    Literal(u64),
    Operator(u8, Vec<Packet>),
}

impl Payload {
    fn new(it: &mut DataIterator) -> Self {
        match it.bits(3) {
            4 => Self::new_number(it),
            type_id => Self::new_operator(it, type_id as u8),
        }
    }

    fn new_number(it: &mut DataIterator) -> Self {
        let mut n = 0;
        while it.next().unwrap() {
            n = (n | it.bits(4)) << 4;
        }
        Payload::Literal(n | it.bits(4))
    }

    fn new_operator(it: &mut DataIterator, type_id: u8) -> Self {
        let packets = if it.bits(1) == 0 {
            let next_index = it.bits(15) as usize + it.index;
            std::iter::from_fn(|| (it.index < next_index).then(|| Packet::new(it))).collect()
        } else {
            (0..(it.bits(11) as usize))
                .map(|_| Packet::new(it))
                .collect()
        };
        Payload::Operator(type_id, packets)
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

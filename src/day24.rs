use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, one_of},
    combinator::map,
    sequence::{preceded, separated_pair},
    Finish, IResult,
};
use std::{collections::HashMap, str::FromStr};

#[derive(Clone, Copy)]
enum Value {
    Register(usize),
    Literal(i32),
}

impl Value {
    fn parse_reg(input: &str) -> IResult<&str, usize> {
        map(one_of("wxyz"), |c| (c as u8 - b'w') as usize)(input)
    }

    fn parse(input: &str) -> IResult<&str, Value> {
        use nom::character::complete::i32;
        let reg = |i| map(Self::parse_reg, Value::Register)(i);
        let lit = |i| map(i32, Value::Literal)(i);
        alt((reg, lit))(input)
    }
}

enum Ins {
    Inp(usize),
    Add(usize, Value),
    Mul(usize, Value),
    Div(usize, Value),
    Mod(usize, Value),
    Eql(usize, Value),
}

impl Ins {
    fn parse(input: &str) -> IResult<&str, Ins> {
        let op = |i| {
            preceded(char(' '), |i| {
                separated_pair(Value::parse_reg, char(' '), Value::parse)(i)
            })(i)
        };
        let inp = map(preceded(tag("inp "), Value::parse_reg), |r| {
            Ins::Inp(r as usize)
        });
        let add = map(preceded(tag("add"), op), |(r, v)| Ins::Add(r, v));
        let mul = map(preceded(tag("mul"), op), |(r, v)| Ins::Mul(r, v));
        let div = map(preceded(tag("div"), op), |(r, v)| Ins::Div(r, v));
        let mod_ = map(preceded(tag("mod"), op), |(r, v)| Ins::Mod(r, v));
        let eql = map(preceded(tag("eql"), op), |(r, v)| Ins::Eql(r, v));
        alt((inp, add, mul, div, mod_, eql))(input)
    }
}

impl FromStr for Ins {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Ins::parse(s).finish() {
            Ok((_, ins)) => Ok(ins),
            Err(nom::error::Error { input, code }) => Err(nom::error::Error {
                input: input.to_owned(),
                code,
            }),
        }
    }
}

#[derive(Default)]
struct Machine {
    regs: [i32; 4],
    smallest: bool,
}

impl Machine {
    fn as_int(&self, v: &Value) -> i32 {
        match v {
            Value::Literal(i) => *i,
            Value::Register(r) => self.regs[*r],
        }
    }

    fn solve(
        &mut self,
        ins: &[Ins],
        idx: usize,
        cache: &mut HashMap<(usize, i32), Option<String>>,
    ) -> Option<String> {
        let z = self.regs[3];
        if let Some(r) = cache.get(&(idx, z)) {
            return r.clone();
        }
        let mut digits = (1..=9).collect::<Vec<_>>();
        if !self.smallest {
            digits.reverse();
        }
        'outer: for digit in digits {
            self.regs = [digit, 0, 0, z];
            for i in idx..ins.len() {
                match &ins[i] {
                    Ins::Inp(_) => {
                        if let Some(a) = self.solve(ins, i + 1, cache) {
                            let r = Some(format!("{}{}", digit, a));
                            cache.insert((idx, z), r.clone());
                            return r;
                        }
                        continue 'outer;
                    }
                    Ins::Add(r, v) => self.regs[*r] += self.as_int(v),
                    Ins::Mul(r, v) => self.regs[*r] *= self.as_int(v),
                    Ins::Div(r, v) => self.regs[*r] /= self.as_int(v),
                    Ins::Mod(r, v) => self.regs[*r] %= self.as_int(v),
                    Ins::Eql(r, v) => self.regs[*r] = (self.regs[*r] == self.as_int(v)) as i32,
                }
            }
            if self.regs[3] == 0 {
                let r = Some(format!("{}", digit));
                cache.insert((idx, z), r.clone());
                return r;
            }
        }
        cache.insert((idx, z), None);
        None
    }
}

#[aoc(day24, part1)]
fn part1(ins: &[Ins]) -> String {
    Machine::default()
        .solve(ins, 1, &mut HashMap::new())
        .unwrap()
}

#[aoc(day24, part2)]
fn part2(ins: &[Ins]) -> String {
    Machine {
        smallest: true,
        ..Default::default()
    }
    .solve(ins, 1, &mut HashMap::new())
    .unwrap()
}

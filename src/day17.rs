use anyhow::Result;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    sequence::{preceded, separated_pair},
    Finish,
};
use std::collections::BTreeMap;

#[allow(clippy::type_complexity)]
fn parser(input: &[u8]) -> Result<((i64, i64), (i64, i64)), nom::error::Error<&[u8]>> {
    use nom::character::complete::i64;
    let bounds = |i| separated_pair(i64, tag(b".."), i64)(i);
    preceded(
        tag(b"target area: x="),
        separated_pair(bounds, tag(b", y="), bounds),
    )(input)
    .map(|(_, bounds)| bounds)
    .finish()
}

fn acceptable(input: &[u8]) -> BTreeMap<i64, Vec<(i64, i64)>> {
    let ((x_min, x_max), (y_min, y_max)) = parser(input).unwrap();
    let (x_min, x_max) = if x_max < 0 {
        (-x_max.max(0), -x_min)
    } else {
        (x_min.max(0), x_max)
    };
    let mut acceptable = BTreeMap::new();
    let dampen = |v, s| v * s - s * (s - 1) / 2;
    for vy in y_min..=y_min.abs().max(y_max) {
        let mut max_for_vy = 0;
        for steps in 1.. {
            let y = dampen(vy, steps);
            max_for_vy = max_for_vy.max(y);
            if y < y_min.min(max_for_vy) {
                break;
            }
            if (y_min..=y_max).contains(&y) {
                acceptable
                    .entry(max_for_vy)
                    .or_insert_with(Vec::new)
                    .extend((1..=x_max).filter_map(|vx| {
                        (x_min..=x_max)
                            .contains(&dampen(vx, steps.min(vx)))
                            .then(|| (vx, vy))
                    }));
            }
        }
    }
    acceptable
}

#[aoc(day17, part1)]
fn part1(input: &[u8]) -> i64 {
    acceptable(input).keys().rev().cloned().next().unwrap()
}

#[aoc(day17, part2)]
fn part2(input: &[u8]) -> usize {
    acceptable(input).values().flatten().unique().count()
}
